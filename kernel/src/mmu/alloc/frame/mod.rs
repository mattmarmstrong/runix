use bootloader_api::BootInfo;

use self::boot::BootFrameAllocator;
use crate::mmu::address::VirtualAddress;
use crate::mmu::vmm::frame::PhysicalFrame;

pub mod boot;
pub mod buddy;

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysicalFrame>;
    fn deallocate_frame(&self, physical_frame: PhysicalFrame);
}

pub(super) fn map_frames(boot_info: &'static BootInfo, start: usize, size: usize) {
    let mut boot_allocator = BootFrameAllocator::new(&boot_info.memory_regions);
    let start_addr = VirtualAddress::with_kernel_base_offset(start);
    let end_addr = start_addr + size;
    unsafe { boot_allocator.allocate_region(start_addr, end_addr) }
}
