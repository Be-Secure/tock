// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: MIT OR Apache-2.0

fn main() {
    println!("cargo:rerun-if-changed=layout.ld");
    println!("cargo:rerun-if-changed=../../kernel_layout.ld");
    println!("cargo:rerun-if-changed=../nrf52840_chip_layout.ld");
}
