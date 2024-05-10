use bootloader_api::info::Optional;
use bootloader_api::BootInfo;

use crate::acpi::read_acpi_tables;
use crate::boot::framebuffer::init_kernel_logging;

pub mod framebuffer;

pub fn init(boot_info: &'static mut BootInfo) {
    let framebuffer = core::mem::replace(&mut boot_info.framebuffer, Optional::None)
        .into_option()
        .unwrap();
    init_kernel_logging(framebuffer);
    let rsdp_addr = core::mem::replace(&mut boot_info.rsdp_addr, Optional::None)
        .into_option()
        .unwrap() as usize;
    read_acpi_tables(rsdp_addr);
}
