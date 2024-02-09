use crate::mmu::vmm::page_table::PageTable;
use crate::mmu::{
    virtual_address::VirtualAddress,
    PhysicalAddress,
};

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum PhysicalFrameSize {}

impl PhysicalFrameSize {
    pub const FOUR_KIB: u64 = 1 << 12;
    pub const TWO_MB: u64 = (1 << 20) * 2;
    pub const ONE_GB: u64 = 1 << 30;
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PhysicalFrame {
    offset: PhysicalAddress,
}

impl PhysicalFrame {
    pub fn raw_from_address_aligned(raw_physical_address: u64) -> Self {
        let aligned_raw_address = !(PhysicalFrameSize::FOUR_KIB - 1) & raw_physical_address;
        PhysicalFrame {
            offset: PhysicalAddress::new(aligned_raw_address),
        }
    }

    // yields the frame that contains the physical address
    pub fn from_address_aligned(physical_address: PhysicalAddress) -> Self {
        let aligned_address = physical_address.align_down(PhysicalFrameSize::FOUR_KIB);
        PhysicalFrame {
            offset: aligned_address,
        }
    }

    pub fn frame_to_page_table(&self, offset: VirtualAddress) -> PageTable {
        let raw_virtual_address = offset.inner + self.offset.inner;
        unsafe { *(raw_virtual_address as *const PageTable) }
    }

    pub fn start_address(&self) -> u64 {
        self.offset.inner
    }
}

#[derive(Debug)]
pub struct PhysicalFrameRange {}
