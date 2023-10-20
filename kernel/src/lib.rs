#![no_std]
#![feature(naked_functions)]
#![feature(asm_const)]

use bootloader_api::BootInfo;
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;

pub mod acpi;
pub mod cpu;
pub mod device;
pub mod framebuffer;
pub mod interrupts;
pub mod mmu;
pub mod segmentation;
pub mod syscall;
pub mod util;

pub static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();
