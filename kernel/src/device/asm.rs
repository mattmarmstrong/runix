// This file will store the shared asm we can use across the device files
// Device-specific asm should go in the corresponding device file

use core::arch::asm;

pub unsafe fn check_cpuid() {
    asm!("nop");
}
