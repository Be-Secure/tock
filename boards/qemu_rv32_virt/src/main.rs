// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Board file for qemu-system-riscv32 "virt" machine type

#![no_std]
// Disable this attribute when documenting, as a workaround for
// https://github.com/rust-lang/rust/issues/62184.
#![cfg_attr(not(doc), no_main)]

use capsules::virtual_alarm::{MuxAlarm, VirtualMuxAlarm};
use kernel::capabilities;
use kernel::component::Component;
use kernel::dynamic_deferred_call::{DynamicDeferredCall, DynamicDeferredCallClientState};
use kernel::hil;
use kernel::platform::scheduler_timer::VirtualSchedulerTimer;
use kernel::platform::KernelResources;
use kernel::platform::SyscallDriverLookup;
use kernel::scheduler::cooperative::CooperativeSched;
use kernel::utilities::registers::interfaces::ReadWriteable;
use kernel::{create_capability, debug, static_init};
use qemu_rv32_virt_chip::chip::{QemuRv32VirtChip, QemuRv32VirtDefaultPeripherals};
use rv32i::csr;

pub mod io;

pub const NUM_PROCS: usize = 4;

// Actual memory for holding the active process structures. Need an empty list
// at least.
static mut PROCESSES: [Option<&'static dyn kernel::process::Process>; NUM_PROCS] =
    [None; NUM_PROCS];

// Reference to the chip for panic dumps.
static mut CHIP: Option<&'static QemuRv32VirtChip<QemuRv32VirtDefaultPeripherals>> = None;

// Reference to the process printer for panic dumps.
static mut PROCESS_PRINTER: Option<&'static kernel::process::ProcessPrinterText> = None;

// How should the kernel respond when a process faults.
const FAULT_RESPONSE: kernel::process::PanicFaultPolicy = kernel::process::PanicFaultPolicy {};

/// Dummy buffer that causes the linker to reserve enough space for the stack.
#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x8000] = [0; 0x8000];

/// A structure representing this platform that holds references to all
/// capsules for this platform. We've included an alarm and console.
struct QemuRv32VirtPlatform {
    console: &'static capsules::console::Console<'static>,
    lldb: &'static capsules::low_level_debug::LowLevelDebug<
        'static,
        capsules::virtual_uart::UartDevice<'static>,
    >,
    alarm: &'static capsules::alarm::AlarmDriver<
        'static,
        VirtualMuxAlarm<'static, qemu_rv32_virt_chip::chip::QemuRv32VirtClint<'static>>,
    >,
    ipc: kernel::ipc::IPC<{ NUM_PROCS as u8 }>,
    scheduler: &'static CooperativeSched<'static>,
    scheduler_timer: &'static VirtualSchedulerTimer<
        VirtualMuxAlarm<'static, qemu_rv32_virt_chip::chip::QemuRv32VirtClint<'static>>,
    >,
    virtio_rng: Option<&'static capsules::rng::RngDriver<'static>>,
}

/// Mapping of integer syscalls to objects that implement syscalls.
impl SyscallDriverLookup for QemuRv32VirtPlatform {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&dyn kernel::syscall::SyscallDriver>) -> R,
    {
        match driver_num {
            capsules::console::DRIVER_NUM => f(Some(self.console)),
            capsules::alarm::DRIVER_NUM => f(Some(self.alarm)),
            capsules::low_level_debug::DRIVER_NUM => f(Some(self.lldb)),
            capsules::rng::DRIVER_NUM => {
                if let Some(rng_driver) = self.virtio_rng {
                    f(Some(rng_driver))
                } else {
                    f(None)
                }
            }
            kernel::ipc::DRIVER_NUM => f(Some(&self.ipc)),
            _ => f(None),
        }
    }
}

impl
    KernelResources<
        qemu_rv32_virt_chip::chip::QemuRv32VirtChip<
            'static,
            QemuRv32VirtDefaultPeripherals<'static>,
        >,
    > for QemuRv32VirtPlatform
{
    type SyscallDriverLookup = Self;
    type SyscallFilter = ();
    type ProcessFault = ();
    type CredentialsCheckingPolicy = ();
    type Scheduler = CooperativeSched<'static>;
    type SchedulerTimer = VirtualSchedulerTimer<
        VirtualMuxAlarm<'static, qemu_rv32_virt_chip::chip::QemuRv32VirtClint<'static>>,
    >;
    type WatchDog = ();
    type ContextSwitchCallback = ();

    fn syscall_driver_lookup(&self) -> &Self::SyscallDriverLookup {
        &self
    }
    fn syscall_filter(&self) -> &Self::SyscallFilter {
        &()
    }
    fn process_fault(&self) -> &Self::ProcessFault {
        &()
    }
    fn credentials_checking_policy(&self) -> &'static Self::CredentialsCheckingPolicy {
        &()
    }
    fn scheduler(&self) -> &Self::Scheduler {
        self.scheduler
    }
    fn scheduler_timer(&self) -> &Self::SchedulerTimer {
        &self.scheduler_timer
    }
    fn watchdog(&self) -> &Self::WatchDog {
        &()
    }
    fn context_switch_callback(&self) -> &Self::ContextSwitchCallback {
        &()
    }
}

/// Main function.
///
/// This function is called from the arch crate after some very basic
/// RISC-V setup and RAM initialization.
#[no_mangle]
pub unsafe fn main() {
    // ---------- BASIC INITIALIZATION -----------

    // Basic setup of the RISC-V IMAC platform
    rv32i::configure_trap_handler(rv32i::PermissionMode::Machine);

    // Acquire required capabilities
    let process_mgmt_cap = create_capability!(capabilities::ProcessManagementCapability);
    let memory_allocation_cap = create_capability!(capabilities::MemoryAllocationCapability);
    let main_loop_cap = create_capability!(capabilities::MainLoopCapability);

    // Create a board kernel instance
    let board_kernel = static_init!(kernel::Kernel, kernel::Kernel::new(&PROCESSES));

    // Some capsules require a callback from a different stack
    // frame. The dynamic deferred call infrastructure can be used to
    // request such a callback (issued from the scheduler) without
    // requiring to wire these capsule up in the chip crates.
    let dynamic_deferred_call_clients =
        static_init!([DynamicDeferredCallClientState; 2], Default::default());
    let dynamic_deferred_caller = static_init!(
        DynamicDeferredCall,
        DynamicDeferredCall::new(dynamic_deferred_call_clients)
    );
    DynamicDeferredCall::set_global_instance(dynamic_deferred_caller);

    // ---------- QEMU-SYSTEM-RISCV32 "virt" MACHINE PERIPHERALS ----------

    let peripherals = static_init!(
        QemuRv32VirtDefaultPeripherals,
        QemuRv32VirtDefaultPeripherals::new(),
    );

    // Create a shared UART channel for the console and for kernel
    // debug over the provided memory-mapped 16550-compatible
    // UART.
    let uart_mux = components::console::UartMuxComponent::new(
        &peripherals.uart0,
        115200,
        dynamic_deferred_caller,
    )
    .finalize(components::uart_mux_component_static!());

    // Use the RISC-V machine timer timesource
    let hardware_timer = static_init!(
        qemu_rv32_virt_chip::chip::QemuRv32VirtClint,
        qemu_rv32_virt_chip::chip::QemuRv32VirtClint::new(&qemu_rv32_virt_chip::clint::CLINT_BASE)
    );

    // Create a shared virtualization mux layer on top of a single hardware
    // alarm.
    let mux_alarm = static_init!(
        MuxAlarm<'static, qemu_rv32_virt_chip::chip::QemuRv32VirtClint>,
        MuxAlarm::new(hardware_timer)
    );
    hil::time::Alarm::set_alarm_client(hardware_timer, mux_alarm);

    // Virtual alarm for the scheduler
    let systick_virtual_alarm = static_init!(
        VirtualMuxAlarm<'static, qemu_rv32_virt_chip::chip::QemuRv32VirtClint>,
        VirtualMuxAlarm::new(mux_alarm)
    );
    systick_virtual_alarm.setup();

    // Virtual alarm and driver for userspace
    let virtual_alarm_user = static_init!(
        VirtualMuxAlarm<'static, qemu_rv32_virt_chip::chip::QemuRv32VirtClint>,
        VirtualMuxAlarm::new(mux_alarm)
    );
    virtual_alarm_user.setup();

    let alarm = static_init!(
        capsules::alarm::AlarmDriver<
            'static,
            VirtualMuxAlarm<'static, qemu_rv32_virt_chip::chip::QemuRv32VirtClint>,
        >,
        capsules::alarm::AlarmDriver::new(
            virtual_alarm_user,
            board_kernel.create_grant(capsules::alarm::DRIVER_NUM, &memory_allocation_cap)
        )
    );
    hil::time::Alarm::set_alarm_client(virtual_alarm_user, alarm);

    // ---------- VIRTIO PERIPHERAL DISCOVERY ----------
    //
    // This board has 8 virtio-mmio (v2 personality required!) devices
    //
    // Collect supported VirtIO peripheral indicies and initialize them if they
    // are found. If there are two instances of a supported peripheral, the one
    // on a higher-indexed VirtIO transport is used.
    let (mut virtio_net_idx, mut virtio_rng_idx) = (None, None);
    for (i, virtio_device) in peripherals.virtio_mmio.iter().enumerate() {
        use qemu_rv32_virt_chip::virtio::devices::VirtIODeviceType;
        match virtio_device.query() {
            Some(VirtIODeviceType::NetworkCard) => {
                virtio_net_idx = Some(i);
            }
            Some(VirtIODeviceType::EntropySource) => {
                virtio_rng_idx = Some(i);
            }
            _ => (),
        }
    }

    // If there is a VirtIO EntropySource present, use the appropriate VirtIORng
    // driver and expose it to userspace though the RngDriver
    let virtio_rng_driver: Option<&'static capsules::rng::RngDriver<'static>> =
        if let Some(rng_idx) = virtio_rng_idx {
            use kernel::hil::rng::Rng;
            use qemu_rv32_virt_chip::virtio::devices::virtio_rng::VirtIORng;
            use qemu_rv32_virt_chip::virtio::queues::split_queue::{
                SplitVirtqueue, VirtqueueAvailableRing, VirtqueueDescriptors, VirtqueueUsedRing,
            };
            use qemu_rv32_virt_chip::virtio::queues::Virtqueue;
            use qemu_rv32_virt_chip::virtio::transports::VirtIOTransport;

            // EntropySource requires a single Virtqueue for retrieved entropy
            let descriptors =
                static_init!(VirtqueueDescriptors<1>, VirtqueueDescriptors::default(),);
            let available_ring =
                static_init!(VirtqueueAvailableRing<1>, VirtqueueAvailableRing::default(),);
            let used_ring = static_init!(VirtqueueUsedRing<1>, VirtqueueUsedRing::default(),);
            let queue = static_init!(
                SplitVirtqueue<1>,
                SplitVirtqueue::new(descriptors, available_ring, used_ring),
            );
            queue.set_transport(&peripherals.virtio_mmio[rng_idx]);

            // VirtIO EntropySource device driver instantiation
            let rng = static_init!(VirtIORng, VirtIORng::new(queue, dynamic_deferred_caller));
            rng.set_deferred_call_handle(
                dynamic_deferred_caller
                    .register(rng)
                    .expect("no deferred call slot available for VirtIO RNG"),
            );
            queue.set_client(rng);

            // Register the queues and driver with the transport, so interrupts
            // are routed properly
            let mmio_queues = static_init!([&'static dyn Virtqueue; 1], [queue; 1]);
            peripherals.virtio_mmio[rng_idx]
                .initialize(rng, mmio_queues)
                .unwrap();

            // Provide an internal randomness buffer
            let rng_buffer = static_init!([u8; 64], [0; 64]);
            rng.provide_buffer(rng_buffer)
                .expect("rng: providing initial buffer failed");

            // Userspace RNG driver over the VirtIO EntropySource
            let rng_driver: &'static mut capsules::rng::RngDriver = static_init!(
                capsules::rng::RngDriver,
                capsules::rng::RngDriver::new(
                    rng,
                    board_kernel.create_grant(capsules::rng::DRIVER_NUM, &memory_allocation_cap),
                ),
            );
            rng.set_client(rng_driver);

            Some(rng_driver as &'static capsules::rng::RngDriver)
        } else {
            // No VirtIO EntropySource discovered
            None
        };

    // If there is a VirtIO NetworkCard present, use the appropriate VirtIONet
    // driver. Currently this is not used, as work on the userspace network
    // driver and kernel network stack is in progress.
    //
    // A template dummy driver is provided to verify basic functionality of this
    // interface.
    let _virtio_net_if: Option<
        &'static qemu_rv32_virt_chip::virtio::devices::virtio_net::VirtIONet<'static>,
    > = if let Some(net_idx) = virtio_net_idx {
        use qemu_rv32_virt_chip::virtio::devices::virtio_net::VirtIONet;
        use qemu_rv32_virt_chip::virtio::queues::split_queue::{
            SplitVirtqueue, VirtqueueAvailableRing, VirtqueueDescriptors, VirtqueueUsedRing,
        };
        use qemu_rv32_virt_chip::virtio::queues::Virtqueue;
        use qemu_rv32_virt_chip::virtio::transports::VirtIOTransport;

        // A VirtIO NetworkCard requires 2 Virtqueues:
        // - a TX Virtqueue with buffers for outgoing packets
        // - a RX Virtqueue where incoming packet buffers are
        //   placed and filled by the device

        // TX Virtqueue
        let tx_descriptors =
            static_init!(VirtqueueDescriptors<2>, VirtqueueDescriptors::default(),);
        let tx_available_ring =
            static_init!(VirtqueueAvailableRing<2>, VirtqueueAvailableRing::default(),);
        let tx_used_ring = static_init!(VirtqueueUsedRing<2>, VirtqueueUsedRing::default(),);
        let tx_queue = static_init!(
            SplitVirtqueue<2>,
            SplitVirtqueue::new(tx_descriptors, tx_available_ring, tx_used_ring),
        );
        tx_queue.set_transport(&peripherals.virtio_mmio[net_idx]);

        // RX Virtqueue
        let rx_descriptors =
            static_init!(VirtqueueDescriptors<2>, VirtqueueDescriptors::default(),);
        let rx_available_ring =
            static_init!(VirtqueueAvailableRing<2>, VirtqueueAvailableRing::default(),);
        let rx_used_ring = static_init!(VirtqueueUsedRing<2>, VirtqueueUsedRing::default(),);
        let rx_queue = static_init!(
            SplitVirtqueue<2>,
            SplitVirtqueue::new(rx_descriptors, rx_available_ring, rx_used_ring),
        );
        rx_queue.set_transport(&peripherals.virtio_mmio[net_idx]);

        // Incoming and outgoing packets are prefixed by a 12-byte
        // VirtIO specific header
        let tx_header_buf = static_init!([u8; 12], [0; 12]);
        let rx_header_buf = static_init!([u8; 12], [0; 12]);

        // Currently, provide a single receive buffer to write
        // incoming packets into
        let rx_buffer = static_init!([u8; 1526], [0; 1526]);

        // Instantiate the VirtIONet (NetworkCard) driver and set
        // the queues
        let virtio_net = static_init!(
            VirtIONet<'static>,
            VirtIONet::new(
                0,
                tx_queue,
                tx_header_buf,
                rx_queue,
                rx_header_buf,
                rx_buffer,
            ),
        );
        tx_queue.set_client(virtio_net);
        rx_queue.set_client(virtio_net);

        // Register the queues and driver with the transport, so
        // interrupts are routed properly
        let mmio_queues = static_init!([&'static dyn Virtqueue; 2], [rx_queue, tx_queue]);
        peripherals.virtio_mmio[net_idx]
            .initialize(virtio_net, mmio_queues)
            .unwrap();

        // Don't forget to enable RX once when integrating this into a
        // proper Ethernet stack:
        // virtio_net.enable_rx();

        // TODO: When we have a proper Ethernet driver available for userspace,
        // return that. For now, just return a reference to the raw VirtIONet
        // driver:
        Some(virtio_net as &'static VirtIONet)
    } else {
        // No VirtIO NetworkCard discovered
        None
    };

    // ---------- INITIALIZE CHIP, ENABLE INTERRUPTS ---------

    let chip = static_init!(
        QemuRv32VirtChip<QemuRv32VirtDefaultPeripherals>,
        QemuRv32VirtChip::new(peripherals, hardware_timer),
    );
    CHIP = Some(chip);

    // Need to enable all interrupts for Tock Kernel
    chip.enable_plic_interrupts();

    // enable interrupts globally
    csr::CSR
        .mie
        .modify(csr::mie::mie::mext::SET + csr::mie::mie::msoft::SET + csr::mie::mie::mtimer::SET);
    csr::CSR.mstatus.modify(csr::mstatus::mstatus::mie::SET);

    // ---------- FINAL SYSTEM INITIALIZATION ----------

    // Create the process printer used in panic prints, etc.
    let process_printer = components::process_printer::ProcessPrinterTextComponent::new()
        .finalize(components::process_printer_text_component_static!());
    PROCESS_PRINTER = Some(process_printer);

    // Setup the console.
    let console = components::console::ConsoleComponent::new(
        board_kernel,
        capsules::console::DRIVER_NUM,
        uart_mux,
    )
    .finalize(components::console_component_static!());
    // Create the debugger object that handles calls to `debug!()`.
    components::debug_writer::DebugWriterComponent::new(uart_mux)
        .finalize(components::debug_writer_component_static!());

    let lldb = components::lldb::LowLevelDebugComponent::new(
        board_kernel,
        capsules::low_level_debug::DRIVER_NUM,
        uart_mux,
    )
    .finalize(components::low_level_debug_component_static!());

    debug!("QEMU RISC-V 32-bit \"virt\" machine, initialization complete.");
    debug!("Entering main loop.");

    // These symbols are defined in the linker script.
    extern "C" {
        /// Beginning of the ROM region containing app images.
        static _sapps: u8;
        /// End of the ROM region containing app images.
        static _eapps: u8;
        /// Beginning of the RAM region for app memory.
        static mut _sappmem: u8;
        /// End of the RAM region for app memory.
        static _eappmem: u8;
    }

    let scheduler = components::sched::cooperative::CooperativeComponent::new(&PROCESSES)
        .finalize(components::cooperative_component_static!(NUM_PROCS));

    let scheduler_timer = static_init!(
        VirtualSchedulerTimer<
            VirtualMuxAlarm<'static, qemu_rv32_virt_chip::chip::QemuRv32VirtClint<'static>>,
        >,
        VirtualSchedulerTimer::new(systick_virtual_alarm)
    );

    let platform = QemuRv32VirtPlatform {
        console,
        alarm,
        lldb,
        scheduler,
        scheduler_timer,
        virtio_rng: virtio_rng_driver,
        ipc: kernel::ipc::IPC::new(
            board_kernel,
            kernel::ipc::DRIVER_NUM,
            &memory_allocation_cap,
        ),
    };

    // ---------- PROCESS LOADING, SCHEDULER LOOP ----------

    kernel::process::load_processes(
        board_kernel,
        chip,
        core::slice::from_raw_parts(
            &_sapps as *const u8,
            &_eapps as *const u8 as usize - &_sapps as *const u8 as usize,
        ),
        core::slice::from_raw_parts_mut(
            &mut _sappmem as *mut u8,
            &_eappmem as *const u8 as usize - &_sappmem as *const u8 as usize,
        ),
        &mut PROCESSES,
        &FAULT_RESPONSE,
        &process_mgmt_cap,
    )
    .unwrap_or_else(|err| {
        debug!("Error loading processes!");
        debug!("{:?}", err);
    });

    board_kernel.kernel_loop(&platform, chip, Some(&platform.ipc), &main_loop_cap);
}
