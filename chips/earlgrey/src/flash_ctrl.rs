// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: MIT OR Apache-2.0

use kernel::utilities::StaticRef;
use lowrisc::flash_ctrl::FlashCtrlRegisters;

pub const FLASH_CTRL_BASE: StaticRef<FlashCtrlRegisters> =
    unsafe { StaticRef::new(0x4100_0000 as *const FlashCtrlRegisters) };
