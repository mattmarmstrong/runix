use bootloader_api::BootInfo;

use self::boot::BootFrameAllocator;
use crate::mmu::address::VirtualAddress;
use crate::mmu::vmm::frame::PhysicalFrame;
use crate::mmu::vmm::page_table::PageTable;

pub mod boot;
pub mod buddy;

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysicalFrame>;
    fn deallocate_frame(&self, physical_frame: PhysicalFrame);
}

pub(super) fn map_frames(boot_info: &'static BootInfo, start: usize, size: usize) {
    log::info!("Mapping initial kernel heap frames");
    let mut boot_allocator = BootFrameAllocator::new(&boot_info.memory_regions);
    let start_addr = VirtualAddress::with_kernel_base_offset(start);
    let end_addr = start_addr + size;
    unsafe { boot_allocator.allocate_region(start_addr, end_addr) }
    log::info!("Kernel heap frames mapped");
    unsafe {
        let pml4 = PageTable::get_active_pml4();
        for entry in pml4.iter_mut() {
            if entry.inner != 0 {
                log::info!("{:#X}", entry.inner);
            };
        }
    }
}
