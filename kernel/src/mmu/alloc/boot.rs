use bootloader_api::info::{
    MemoryRegionKind,
    MemoryRegions,
};
use spin::Mutex;

use crate::mmu::vmm::frame::{
    PhysicalFrame,
    PhysicalFrameSize,
};

pub const KERNEL_HEAP_START: u64 = 0xDEAD_BEEF;
pub const KERNEL_HEAP_SIZE: u64 = 1 << 21; // 1 MB

#[derive(Debug)]
#[repr(C)]
pub struct BootFrameAllocator {
    memory_regions: Mutex<&'static MemoryRegions>,
}

impl BootFrameAllocator {
    pub fn new(memory_regions: &'static MemoryRegions) -> Self {
        let memory_regions = Mutex::new(memory_regions);
        BootFrameAllocator { memory_regions }
    }

    pub fn usable_frames(&self) -> impl Iterator<Item = PhysicalFrame> {
        self.memory_regions
            .lock()
            .iter()
            .filter(|memory_region| memory_region.kind == MemoryRegionKind::Usable)
            .map(|usable_region| usable_region.start..usable_region.end)
            .flat_map(|address_range| address_range.step_by(PhysicalFrameSize::FOUR_KIB as usize))
            .map(|frame_start_address| PhysicalFrame::raw_from_address_aligned(frame_start_address))
    }
}
