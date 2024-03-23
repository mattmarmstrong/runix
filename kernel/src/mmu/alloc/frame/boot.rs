use bootloader_api::info::{
    MemoryRegionKind,
    MemoryRegions,
};
use spin::Mutex;

use crate::mmu::address::VirtualAddress;
use crate::mmu::alloc::frame::FrameAllocator;
use crate::mmu::vmm::frame::PhysicalFrame;
use crate::mmu::vmm::page::{
    VirtualPage,
    VirtualPageRange,
};
use crate::mmu::vmm::page_table::MappedPageTable;
use crate::mmu::vmm::page_table::PageTable;
use crate::mmu::vmm::page_table_entry::PageTableEntryFlags;
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
            .flat_map(|address_range| address_range.step_by(Size::FOUR_KIB))
            .map(|frame_start_address| PhysicalFrame::from_raw_address_aligned(frame_start_address as usize))
    }

    // This is intended to be used exclusively in a kernel context, shortly after booting.
    pub unsafe fn allocate_region(&mut self, start_addr: VirtualAddress, end_addr: VirtualAddress) {
        let start_page = VirtualPage::from_address_aligned(start_addr);
        let end_page = VirtualPage::from_address_aligned(end_addr);
        let page_range = VirtualPageRange::range_inclusive(start_page, end_page);
        let mut active_pml4 = MappedPageTable::new(VirtualAddress::kernel_base(), PageTable::get_active_pml4());
        page_range.for_each(|page| {
            let allocated_frame = self.allocate_frame().unwrap();
            let entry_flags = PageTableEntryFlags::PRESENT & PageTableEntryFlags::WRITE_ACCESS;
            let table_flags = entry_flags;
            active_pml4.map_to(page, allocated_frame, entry_flags, table_flags, true, self)
        })
    }
}
