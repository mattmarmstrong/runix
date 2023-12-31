#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

use bootloader_api::{
    config::Mapping,
    entry_point,
    BootInfo,
    BootloaderConfig,
};

const BOOTLOADER_CONFIG: BootloaderConfig = {
    let physical_mapping_offset = kernel::mmu::KERNEL_BASE_ADDRESS;
    let kernel_stack_size: u64 = 1024 * 1024;
    let mut boot_config = BootloaderConfig::new_default();
    boot_config.kernel_stack_size = kernel_stack_size;
    // Put the kernel into the higher-half of the virtual address space
    boot_config.mappings.dynamic_range_start = Some(physical_mapping_offset);
    boot_config.mappings.physical_memory = Some(Mapping::FixedAddress(physical_mapping_offset));
    boot_config
};

entry_point!(kmain, config = &BOOTLOADER_CONFIG);

fn kmain(boot_info: &'static mut BootInfo) -> ! {
    let rsdp_addr = boot_info.rsdp_addr.into_option().unwrap();
    kernel::framebuffer::init_kernel_logging(boot_info);
    kernel::acpi::read_acpi_tables(rsdp_addr);
    kernel::segmentation::init_gdt();
    kernel::interrupts::init_idt();
    kernel::cpu::init_cpu_intrinsics();
    log::info!("{}", kernel::cpu::CPU_INFO.get().unwrap());
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
