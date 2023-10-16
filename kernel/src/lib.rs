#![no_std]
#![feature(naked_functions)]
#![feature(asm_const)]

use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;

pub mod device;
pub mod framebuffer;
pub mod interrupts;
pub mod segmentation;
pub mod syscall;
pub mod util;
pub mod vmm;

pub static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();
