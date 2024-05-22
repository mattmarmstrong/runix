#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader_api::{
    config::Mapping,
    entry_point,
    BootInfo,
    BootloaderConfig,
};

const BOOTLOADER_CONFIG: BootloaderConfig = {
    let physical_mapping_offset = kernel::mmu::KERNEL_BASE_ADDRESS as u64;
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
    kernel::boot::init(boot_info);
    // kernel::cpu::init_cpu_intrinsics();
    log::info!("{}", kernel::cpu::CPU_INFO.get().unwrap());
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
