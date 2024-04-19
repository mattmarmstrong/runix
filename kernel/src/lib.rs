#![no_std]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(allocator_api)]

use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;

pub mod acpi;
pub mod boot;
pub mod cpu;
pub mod device;
pub mod interrupts;
pub mod mmu;
pub mod process;
pub mod segmentation;
pub mod syscall;
pub mod util;

pub static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();
