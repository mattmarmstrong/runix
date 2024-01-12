use super::page_table::PageTable;
use crate::mmu::{
    virtual_address::VirtualAddress,
    PhysicalAddress,
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalFrame {
    size: u64,
    offset: PhysicalAddress,
}

impl PhysicalFrame {
    // yields the frame that contains the physical address
    pub fn from_address_aligned(physical_address: PhysicalAddress, page_size: u64) -> Self {
        let aligned_address = physical_address.align_down(page_size);
        PhysicalFrame {
            size: page_size,
            offset: aligned_address,
        }
    }

    pub fn frame_to_page_table(&self, offset: VirtualAddress) -> PageTable {
        let raw_virtual_address = offset.inner + self.offset.inner;
        unsafe { *(raw_virtual_address as *const PageTable) }
    }
}
