//! Tock kernel for the Arduino Nano RP2040 Connect.
//!
//! It is based on RP2040SoC SoC (Cortex M0+).

#![no_std]
// Disable this attribute when documenting, as a workaround for
// https://github.com/rust-lang/rust/issues/62184.
#![cfg_attr(not(doc), no_main)]
#![deny(missing_docs)]
#![feature(naked_functions)]

use core::arch::asm;

use components::gpio::GpioComponent;
use components::led::LedsComponent;
use core_capsules::virtual_alarm::VirtualMuxAlarm;
use enum_primitive::cast::FromPrimitive;
use kernel::component::Component;
use kernel::debug;
use kernel::dynamic_deferred_call::{DynamicDeferredCall, DynamicDeferredCallClientState};
use kernel::hil::led::LedHigh;
use kernel::platform::{KernelResources, SyscallDriverLookup};
use kernel::scheduler::round_robin::RoundRobinSched;
use kernel::syscall::SyscallDriver;
use kernel::{capabilities, create_capability, static_init, Kernel};
use rp2040;
use rp2040::adc::{Adc, Channel};
use rp2040::chip::{Rp2040, Rp2040DefaultPeripherals};
use rp2040::clocks::{
    AdcAuxiliaryClockSource, PeripheralAuxiliaryClockSource, PllClock,
    ReferenceAuxiliaryClockSource, ReferenceClockSource, RtcAuxiliaryClockSource,
    SystemAuxiliaryClockSource, SystemClockSource, UsbAuxiliaryClockSource,
};
use rp2040::gpio::{GpioFunction, RPGpio, RPGpioPin};
use rp2040::resets::Peripheral;
use rp2040::timer::RPTimer;
mod io;

use rp2040::sysinfo;

mod flash_bootloader;

/// Allocate memory for the stack
#[no_mangle]
#[link_section = ".stack_buffer"]
pub static mut STACK_MEMORY: [u8; 0x1000] = [0; 0x1000];

// Manually setting the boot header section that contains the FCB header
#[used]
#[link_section = ".flash_bootloader"]
static FLASH_BOOTLOADER: [u8; 256] = flash_bootloader::FLASH_BOOTLOADER;

// State for loading and holding applications.
// How should the kernel respond when a process faults.
const FAULT_RESPONSE: kernel::process::PanicFaultPolicy = kernel::process::PanicFaultPolicy {};

// Number of concurrent processes this platform supports.
const NUM_PROCS: usize = 4;

static mut PROCESSES: [Option<&'static dyn kernel::process::Process>; NUM_PROCS] =
    [None; NUM_PROCS];

static mut CHIP: Option<&'static Rp2040<Rp2040DefaultPeripherals>> = None;
static mut PROCESS_PRINTER: Option<&'static kernel::process::ProcessPrinterText> = None;

/// Supported drivers by the platform
pub struct NanoRP2040Connect {
    ipc: kernel::ipc::IPC<{ NUM_PROCS as u8 }>,
    console: &'static core_capsules::console::Console<'static>,
    alarm: &'static core_capsules::alarm::AlarmDriver<
        'static,
        VirtualMuxAlarm<'static, rp2040::timer::RPTimer<'static>>,
    >,
    gpio: &'static core_capsules::gpio::GPIO<'static, RPGpioPin<'static>>,
    led: &'static core_capsules::led::LedDriver<'static, LedHigh<'static, RPGpioPin<'static>>, 1>,
    adc: &'static core_capsules::adc::AdcVirtualized<'static>,
    temperature: &'static extra_capsules::temperature::TemperatureSensor<'static>,
    ninedof: &'static extra_capsules::ninedof::NineDof<'static>,
    lsm6dsoxtr: &'static extra_capsules::lsm6dsoxtr::Lsm6dsoxtrI2C<'static>,

    scheduler: &'static RoundRobinSched<'static>,
    systick: cortexm0p::systick::SysTick,
}

impl SyscallDriverLookup for NanoRP2040Connect {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&dyn SyscallDriver>) -> R,
    {
        match driver_num {
            core_capsules::console::DRIVER_NUM => f(Some(self.console)),
            core_capsules::alarm::DRIVER_NUM => f(Some(self.alarm)),
            core_capsules::gpio::DRIVER_NUM => f(Some(self.gpio)),
            core_capsules::led::DRIVER_NUM => f(Some(self.led)),
            kernel::ipc::DRIVER_NUM => f(Some(&self.ipc)),
            core_capsules::adc::DRIVER_NUM => f(Some(self.adc)),
            extra_capsules::temperature::DRIVER_NUM => f(Some(self.temperature)),
            extra_capsules::lsm6dsoxtr::DRIVER_NUM => f(Some(self.lsm6dsoxtr)),
            extra_capsules::ninedof::DRIVER_NUM => f(Some(self.ninedof)),
            _ => f(None),
        }
    }
}

impl KernelResources<Rp2040<'static, Rp2040DefaultPeripherals<'static>>> for NanoRP2040Connect {
    type SyscallDriverLookup = Self;
    type SyscallFilter = ();
    type ProcessFault = ();
    type CredentialsCheckingPolicy = ();
    type Scheduler = RoundRobinSched<'static>;
    type SchedulerTimer = cortexm0p::systick::SysTick;
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
        &self.systick
    }
    fn watchdog(&self) -> &Self::WatchDog {
        &()
    }
    fn context_switch_callback(&self) -> &Self::ContextSwitchCallback {
        &()
    }
}

/// Entry point used for debuger
///
/// When loaded using gdb, the Arduino Nano RP2040 Connect is not reset
/// by default. Without this function, gdb sets the PC to the
/// beginning of the flash. This is not correct, as the RP2040
/// has a more complex boot process.
///
/// This function is set to be the entry point for gdb and is used
/// to send the RP2040 back in the bootloader so that all the boot
/// sqeuence is performed.
#[no_mangle]
#[naked]
pub unsafe extern "C" fn jump_to_bootloader() {
    asm!(
        "
    movs r0, #0
    ldr r1, =(0xe0000000 + 0x0000ed08)
    str r0, [r1]
    ldmia r0!, {{r1, r2}}
    msr msp, r1
    bx r2
    ",
        options(noreturn)
    );
}

fn init_clocks(peripherals: &Rp2040DefaultPeripherals) {
    // Start tick in watchdog
    peripherals.watchdog.start_tick(12);

    // Disable the Resus clock
    peripherals.clocks.disable_resus();

    // Setup the external Osciallator
    peripherals.xosc.init();

    // disable ref and sys clock aux sources
    peripherals.clocks.disable_sys_aux();
    peripherals.clocks.disable_ref_aux();

    peripherals
        .resets
        .reset(&[Peripheral::PllSys, Peripheral::PllUsb]);
    peripherals
        .resets
        .unreset(&[Peripheral::PllSys, Peripheral::PllUsb], true);

    // Configure PLLs (from Pico SDK)
    //                   REF     FBDIV VCO            POSTDIV
    // PLL SYS: 12 / 1 = 12MHz * 125 = 1500MHZ / 6 / 2 = 125MHz
    // PLL USB: 12 / 1 = 12MHz * 40  = 480 MHz / 5 / 2 =  48MHz

    // It seems that the external osciallator is clocked at 12 MHz

    peripherals
        .clocks
        .pll_init(PllClock::Sys, 12, 1, 1500 * 1000000, 6, 2);
    peripherals
        .clocks
        .pll_init(PllClock::Usb, 12, 1, 480 * 1000000, 5, 2);

    // pico-sdk: // CLK_REF = XOSC (12MHz) / 1 = 12MHz
    peripherals.clocks.configure_reference(
        ReferenceClockSource::Xosc,
        ReferenceAuxiliaryClockSource::PllUsb,
        12000000,
        12000000,
    );
    // pico-sdk: CLK SYS = PLL SYS (125MHz) / 1 = 125MHz
    peripherals.clocks.configure_system(
        SystemClockSource::Auxiliary,
        SystemAuxiliaryClockSource::PllSys,
        125000000,
        125000000,
    );
    // pico-sdk: CLK USB = PLL USB (48MHz) / 1 = 48MHz
    peripherals
        .clocks
        .configure_usb(UsbAuxiliaryClockSource::PllSys, 48000000, 48000000);
    // pico-sdk: CLK ADC = PLL USB (48MHZ) / 1 = 48MHz
    peripherals
        .clocks
        .configure_adc(AdcAuxiliaryClockSource::PllUsb, 48000000, 48000000);
    // pico-sdk: CLK RTC = PLL USB (48MHz) / 1024 = 46875Hz
    peripherals
        .clocks
        .configure_rtc(RtcAuxiliaryClockSource::PllSys, 48000000, 46875);
    // pico-sdk:
    // CLK PERI = clk_sys. Used as reference clock for Peripherals. No dividers so just select and enable
    // Normally choose clk_sys or clk_usb
    peripherals
        .clocks
        .configure_peripheral(PeripheralAuxiliaryClockSource::System, 125000000);
}

/// This is in a separate, inline(never) function so that its stack frame is
/// removed when this function returns. Otherwise, the stack space used for
/// these static_inits is wasted.
#[inline(never)]
unsafe fn create_peripherals() -> &'static mut Rp2040DefaultPeripherals<'static> {
    static_init!(Rp2040DefaultPeripherals, Rp2040DefaultPeripherals::new())
}

/// Main function called after RAM initialized.
#[no_mangle]
pub unsafe fn main() {
    // Loads relocations and clears BSS
    rp2040::init();

    let peripherals = create_peripherals();
    peripherals.resolve_dependencies();

    // Set the UART used for panic
    io::WRITER.set_uart(&peripherals.uart0);

    // Reset all peripherals except QSPI (we might be booting from Flash), PLL USB and PLL SYS
    peripherals.resets.reset_all_except(&[
        Peripheral::IOQSpi,
        Peripheral::PadsQSpi,
        Peripheral::PllUsb,
        Peripheral::PllSys,
    ]);

    // Unreset all the peripherals that do not require clock setup as they run using the sys_clk or ref_clk
    // Wait for the peripherals to reset
    peripherals.resets.unreset_all_except(
        &[
            Peripheral::Adc,
            Peripheral::Rtc,
            Peripheral::Spi0,
            Peripheral::Spi1,
            Peripheral::Uart0,
            Peripheral::Uart1,
            Peripheral::UsbCtrl,
        ],
        true,
    );

    init_clocks(&peripherals);

    // Unreset all peripherals
    peripherals.resets.unreset_all_except(&[], true);

    //set RX and TX pins in UART mode
    let gpio_tx = peripherals.pins.get_pin(RPGpio::GPIO0);
    let gpio_rx = peripherals.pins.get_pin(RPGpio::GPIO1);
    gpio_rx.set_function(GpioFunction::UART);
    gpio_tx.set_function(GpioFunction::UART);
    // Disable IE for pads 26-29 (the Pico SDK runtime does this, not sure why)
    for pin in 26..30 {
        peripherals
            .pins
            .get_pin(RPGpio::from_usize(pin).unwrap())
            .deactivate_pads();
    }

    let chip = static_init!(
        Rp2040<Rp2040DefaultPeripherals>,
        Rp2040::new(peripherals, &peripherals.sio)
    );

    CHIP = Some(chip);

    let board_kernel = static_init!(Kernel, Kernel::new(&PROCESSES));

    let process_management_capability =
        create_capability!(capabilities::ProcessManagementCapability);
    let main_loop_capability = create_capability!(capabilities::MainLoopCapability);
    let memory_allocation_capability = create_capability!(capabilities::MemoryAllocationCapability);

    let dynamic_deferred_call_clients =
        static_init!([DynamicDeferredCallClientState; 2], Default::default());
    let dynamic_deferred_caller = static_init!(
        DynamicDeferredCall,
        DynamicDeferredCall::new(dynamic_deferred_call_clients)
    );
    DynamicDeferredCall::set_global_instance(dynamic_deferred_caller);

    let mux_alarm = components::alarm::AlarmMuxComponent::new(&peripherals.timer)
        .finalize(components::alarm_mux_component_static!(RPTimer));

    let alarm = components::alarm::AlarmDriverComponent::new(
        board_kernel,
        core_capsules::alarm::DRIVER_NUM,
        mux_alarm,
    )
    .finalize(components::alarm_component_static!(RPTimer));

    // UART
    // Create a shared UART channel for kernel debug.
    let uart_mux = components::console::UartMuxComponent::new(
        &peripherals.uart0,
        115200,
        dynamic_deferred_caller,
    )
    .finalize(components::uart_mux_component_static!());

    // Setup the console.
    let console = components::console::ConsoleComponent::new(
        board_kernel,
        core_capsules::console::DRIVER_NUM,
        uart_mux,
    )
    .finalize(components::console_component_static!());
    // Create the debugger object that handles calls to `debug!()`.
    components::debug_writer::DebugWriterComponent::new(uart_mux)
        .finalize(components::debug_writer_component_static!());

    let gpio = GpioComponent::new(
        board_kernel,
        core_capsules::gpio::DRIVER_NUM,
        components::gpio_component_helper!(
            RPGpioPin,
            // Used for serial communication. Comment them in if you don't use serial.
            // 0 => &peripherals.pins.get_pin(RPGpio::GPIO0),
            // 1 => &peripherals.pins.get_pin(RPGpio::GPIO1),
            2 => &peripherals.pins.get_pin(RPGpio::GPIO2),
            3 => &peripherals.pins.get_pin(RPGpio::GPIO3),
            // 4 => &peripherals.pins.get_pin(RPGpio::GPIO4),
            5 => &peripherals.pins.get_pin(RPGpio::GPIO5),
            // 6 => &peripherals.pins.get_pin(RPGpio::GPIO6),
            // 7 => &peripherals.pins.get_pin(RPGpio::GPIO7),
            8 => &peripherals.pins.get_pin(RPGpio::GPIO8),
            9 => &peripherals.pins.get_pin(RPGpio::GPIO9),
            10 => &peripherals.pins.get_pin(RPGpio::GPIO10),
            11 => &peripherals.pins.get_pin(RPGpio::GPIO11),
            // 12 => &peripherals.pins.get_pin(RPGpio::GPIO12),
            // 13 => &peripherals.pins.get_pin(RPGpio::GPIO13),
            14 => &peripherals.pins.get_pin(RPGpio::GPIO14),
            15 => &peripherals.pins.get_pin(RPGpio::GPIO15),
            16 => &peripherals.pins.get_pin(RPGpio::GPIO16),
            17 => &peripherals.pins.get_pin(RPGpio::GPIO17),
            18 => &peripherals.pins.get_pin(RPGpio::GPIO18),
            19 => &peripherals.pins.get_pin(RPGpio::GPIO19),
            20 => &peripherals.pins.get_pin(RPGpio::GPIO20),
            21 => &peripherals.pins.get_pin(RPGpio::GPIO21),
            22 => &peripherals.pins.get_pin(RPGpio::GPIO22),
            23 => &peripherals.pins.get_pin(RPGpio::GPIO23),
            24 => &peripherals.pins.get_pin(RPGpio::GPIO24),
            // LED pin
            // 25 => &peripherals.pins.get_pin(RPGpio::GPIO25),

            // Uncomment to use these as GPIO pins instead of ADC pins
            // 26 => &peripherals.pins.get_pin(RPGpio::GPIO26),
            // 27 => &peripherals.pins.get_pin(RPGpio::GPIO27),
            // 28 => &peripherals.pins.get_pin(RPGpio::GPIO28),
            // 29 => &peripherals.pins.get_pin(RPGpio::GPIO29)
        ),
    )
    .finalize(components::gpio_component_static!(RPGpioPin<'static>));

    let led = LedsComponent::new().finalize(components::led_component_static!(
        LedHigh<'static, RPGpioPin<'static>>,
        LedHigh::new(&peripherals.pins.get_pin(RPGpio::GPIO6))
    ));

    peripherals.adc.init();

    let adc_mux = components::adc::AdcMuxComponent::new(&peripherals.adc)
        .finalize(components::adc_mux_component_static!(Adc));

    let temp_sensor = components::temperature_rp2040::TemperatureRp2040Component::new(
        adc_mux,
        Channel::Channel4,
        1.721,
        0.706,
    )
    .finalize(components::temperature_rp2040_adc_component_static!(
        rp2040::adc::Adc
    ));

    peripherals.i2c0.init(100 * 1000);
    //set SDA and SCL pins in I2C mode
    let gpio_sda = peripherals.pins.get_pin(RPGpio::GPIO12);
    let gpio_scl = peripherals.pins.get_pin(RPGpio::GPIO13);
    gpio_sda.set_function(GpioFunction::I2C);
    gpio_scl.set_function(GpioFunction::I2C);
    let mux_i2c =
        components::i2c::I2CMuxComponent::new(&peripherals.i2c0, None, dynamic_deferred_caller)
            .finalize(components::i2c_mux_component_static!());

    let lsm6dsoxtr = components::lsm6dsox::Lsm6dsoxtrI2CComponent::new(
        mux_i2c,
        extra_capsules::lsm6dsoxtr::ACCELEROMETER_BASE_ADDRESS,
        board_kernel,
        extra_capsules::lsm6dsoxtr::DRIVER_NUM,
    )
    .finalize(components::lsm6ds_i2c_component_static!());

    let ninedof = components::ninedof::NineDofComponent::new(
        board_kernel,
        extra_capsules::ninedof::DRIVER_NUM,
    )
    .finalize(components::ninedof_component_static!(lsm6dsoxtr));

    let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);
    let grant_temperature =
        board_kernel.create_grant(extra_capsules::temperature::DRIVER_NUM, &grant_cap);

    let temp = static_init!(
        extra_capsules::temperature::TemperatureSensor<'static>,
        extra_capsules::temperature::TemperatureSensor::new(temp_sensor, grant_temperature)
    );

    let _ = lsm6dsoxtr
        .configure(
            extra_capsules::lsm6dsoxtr::LSM6DSOXGyroDataRate::LSM6DSOX_GYRO_RATE_12_5_HZ,
            extra_capsules::lsm6dsoxtr::LSM6DSOXAccelDataRate::LSM6DSOX_ACCEL_RATE_12_5_HZ,
            extra_capsules::lsm6dsoxtr::LSM6DSOXAccelRange::LSM6DSOX_ACCEL_RANGE_2_G,
            extra_capsules::lsm6dsoxtr::LSM6DSOXTRGyroRange::LSM6DSOX_GYRO_RANGE_250_DPS,
            true,
        )
        .map_err(|e| {
            panic!(
                "ERROR Failed to start LSM6DSOXTR sensor configuration ({:?})",
                e
            )
        });

    // The Nano_RP2040 board has its own integrated temperature sensor, as well as a temperature sensor integrated in the lsm6dsoxtr sensor.
    // There is only a single driver, thus either for userspace is exclusive.
    // Uncomment this block in order to use the temperature sensor from lsm6dsoxtr

    // let temp = static_init!(
    //     extra_capsules::temperature::TemperatureSensor<'static>,
    //     extra_capsules::temperature::TemperatureSensor::new(lsm6dsoxtr, grant_temperature)
    // );

    kernel::hil::sensors::TemperatureDriver::set_client(temp_sensor, temp);

    let adc_channel_0 = components::adc::AdcComponent::new(&adc_mux, Channel::Channel0)
        .finalize(components::adc_component_static!(Adc));

    let adc_channel_1 = components::adc::AdcComponent::new(&adc_mux, Channel::Channel1)
        .finalize(components::adc_component_static!(Adc));

    let adc_channel_2 = components::adc::AdcComponent::new(&adc_mux, Channel::Channel2)
        .finalize(components::adc_component_static!(Adc));

    let adc_channel_3 = components::adc::AdcComponent::new(&adc_mux, Channel::Channel3)
        .finalize(components::adc_component_static!(Adc));

    let adc_syscall =
        components::adc::AdcVirtualComponent::new(board_kernel, core_capsules::adc::DRIVER_NUM)
            .finalize(components::adc_syscall_component_helper!(
                adc_channel_0,
                adc_channel_1,
                adc_channel_2,
                adc_channel_3,
            ));

    let process_printer = components::process_printer::ProcessPrinterTextComponent::new()
        .finalize(components::process_printer_text_component_static!());
    PROCESS_PRINTER = Some(process_printer);

    // PROCESS CONSOLE
    let process_console = components::process_console::ProcessConsoleComponent::new(
        board_kernel,
        uart_mux,
        mux_alarm,
        process_printer,
    )
    .finalize(components::process_console_component_static!(RPTimer));
    let _ = process_console.start();

    let scheduler = components::sched::round_robin::RoundRobinComponent::new(&PROCESSES)
        .finalize(components::round_robin_component_static!(NUM_PROCS));

    let nano_rp2040_connect = NanoRP2040Connect {
        ipc: kernel::ipc::IPC::new(
            board_kernel,
            kernel::ipc::DRIVER_NUM,
            &memory_allocation_capability,
        ),
        alarm: alarm,
        gpio: gpio,
        led: led,
        console: console,
        adc: adc_syscall,
        temperature: temp,

        lsm6dsoxtr: lsm6dsoxtr,
        ninedof: ninedof,

        scheduler,
        systick: cortexm0p::systick::SysTick::new_with_calibration(125_000_000),
    };

    let platform_type = match peripherals.sysinfo.get_platform() {
        sysinfo::Platform::Asic => "ASIC",
        sysinfo::Platform::Fpga => "FPGA",
    };

    debug!(
        "RP2040 Revision {} {}",
        peripherals.sysinfo.get_revision(),
        platform_type
    );
    debug!("Initialization complete. Enter main loop");

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
        &process_management_capability,
    )
    .unwrap_or_else(|err| {
        debug!("Error loading processes!");
        debug!("{:?}", err);
    });

    board_kernel.kernel_loop(
        &nano_rp2040_connect,
        chip,
        Some(&nano_rp2040_connect.ipc),
        &main_loop_capability,
    );
}
