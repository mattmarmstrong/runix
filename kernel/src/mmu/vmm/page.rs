use crate::mmu::vmm::asm::flush;
use crate::mmu::VirtualAddress;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum VirtualPageSize {}

impl VirtualPageSize {
    pub const FOUR_KIB: u64 = 1 << 12;
    pub const TWO_MB: u64 = (1 << 20) * 2;
    pub const ONE_GB: u64 = 1 << 30;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualPage {
    offset: VirtualAddress,
}

impl core::ops::Add<u64> for VirtualPage {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        VirtualPage::from_address_aligned(self.offset + (rhs * VirtualPageSize::FOUR_KIB))
    }
}

impl core::ops::AddAssign<u64> for VirtualPage {
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + rhs;
    }
}

impl VirtualPage {
    pub fn from_address_aligned(virtual_address: VirtualAddress) -> Self {
        let aligned_address = virtual_address.align_down(VirtualPageSize::FOUR_KIB);
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
