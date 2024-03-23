use crate::mmu::address::{
    PhysicalAddress,
    VirtualAddress,
};
use crate::mmu::vmm::page_table::PageTable;
use crate::mmu::vmm::Size;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct PhysicalFrame {
    pub offset: PhysicalAddress,
}

impl core::ops::Add<usize> for PhysicalFrame {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        PhysicalFrame::from_address_aligned(self.offset + (rhs * Size::FOUR_KIB))
    }
}

impl core::ops::AddAssign<usize> for PhysicalFrame {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

impl PhysicalFrame {
    pub fn from_raw_address_aligned(raw_physical_address: usize) -> Self {
        let aligned_raw_address = !(Size::FOUR_KIB - 1) & raw_physical_address;
        PhysicalFrame {
            offset: PhysicalAddress::new(aligned_raw_address),
        }
    }

    // yields the frame that contains the physical address
    pub fn from_address_aligned(physical_address: PhysicalAddress) -> Self {
        let aligned_address = physical_address.align_down(Size::FOUR_KIB);
        PhysicalFrame {
            offset: aligned_address,
        }
    }

    pub fn frame_to_page_table(&self, offset: VirtualAddress) -> PageTable {
        let raw_virtual_address = offset.inner + self.offset.inner;
        unsafe { *(raw_virtual_address as *const PageTable) }
    }

    pub fn start_address(&self) -> usize {
        self.offset.inner
    }
}

#[derive(Debug)]
pub struct PhysicalFrameRange {
    current: PhysicalFrame,
    end: PhysicalFrame,
}

impl Iterator for PhysicalFrameRange {
    type Item = PhysicalFrame;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current < self.end {
            true => {
                let current_page = self.current;
                self.current += 1;
                Some(current_page)
            }
            false => None,
        }
    }
}

impl PhysicalFrameRange {
    pub fn range_inclusive(start: PhysicalFrame, end: PhysicalFrame) -> Self {
        PhysicalFrameRange { current: start, end }
    }
}
