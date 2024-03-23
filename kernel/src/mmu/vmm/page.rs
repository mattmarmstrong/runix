use crate::mmu::address::VirtualAddress;
use crate::mmu::vmm::asm::flush;
use crate::mmu::vmm::Size;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualPage {
    pub offset: VirtualAddress,
}

impl core::ops::Add<usize> for VirtualPage {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        VirtualPage::from_address_aligned(self.offset + (rhs * Size::FOUR_KIB))
    }
}

impl core::ops::AddAssign<usize> for VirtualPage {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

impl VirtualPage {
    pub fn from_address_aligned(virtual_address: VirtualAddress) -> Self {
        let aligned_address = virtual_address.align_down(Size::FOUR_KIB);
        VirtualPage {
            offset: aligned_address,
        }
    }

    pub fn flush_from_tlb(&self) {
        unsafe { flush(self.offset.inner) }
    }
}

pub struct VirtualPageRange {
    current: VirtualPage,
    end: VirtualPage,
}

impl Iterator for VirtualPageRange {
    type Item = VirtualPage;

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

impl VirtualPageRange {
    pub fn range_inclusive(start: VirtualPage, end: VirtualPage) -> Self {
        VirtualPageRange { current: start, end }
    }
}
