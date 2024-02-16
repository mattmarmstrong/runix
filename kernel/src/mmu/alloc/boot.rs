use bootloader_api::info::{
    MemoryRegionKind,
    MemoryRegions,
};
use spin::Mutex;

use crate::mmu::alloc::FrameAllocator;
use crate::mmu::vmm::frame::PhysicalFrame;
use crate::mmu::vmm::Size;

#[derive(Debug)]
#[repr(C)]
pub struct BootFrameAllocator {
    memory_regions: Mutex<&'static MemoryRegions>,
    index: usize,
}

impl FrameAllocator for BootFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysicalFrame> {
        let frame = self.usable_frames().nth(self.index);
        self.index += 1;
        frame
    }
    fn deallocate_frame(&self, _physical_frame: PhysicalFrame) {
        panic!("Attempted to dealloc!");
    }
}

impl BootFrameAllocator {
    pub fn new(memory_regions: &'static MemoryRegions) -> Self {
        let memory_regions = Mutex::new(memory_regions);
        BootFrameAllocator {
            memory_regions,
            index: 0,
        }
    }

    pub fn usable_frames(&self) -> impl Iterator<Item = PhysicalFrame> {
        self.memory_regions
            .lock()
            .iter()
            .filter(|memory_region| memory_region.kind == MemoryRegionKind::Usable)
            .map(|usable_region| usable_region.start..usable_region.end)
            .flat_map(|address_range| address_range.step_by(Size::FOUR_KIB as usize))
            .map(|frame_start_address| PhysicalFrame::from_raw_address_aligned(frame_start_address))
    }
}
