// memory management software
pub mod alloc;
pub mod physical_address;
pub mod virtual_address;
pub mod vmm;

use crate::mmu::physical_address::PhysicalAddress;
use crate::mmu::virtual_address::VirtualAddress;

pub const KERNEL_BASE_ADDRESS: u64 = 0xFFFF_8880_0000_0000;

// There was absolutely no need to do this. I just wanted to write a macro
#[macro_export]
macro_rules! impl_alignment_functions {
    ($addr_type: ty) => {
        impl $addr_type {
            #[inline]
            pub fn align_down(self, alignment: u64) -> Self {
                debug_assert!(alignment.is_power_of_two());
                if self.inner % (alignment) == 0 {
                    self
                } else {
                    let alignment_mask: u64 = !(alignment - 1);
                    let aligned_address: u64 = self.inner & alignment_mask;
                    Self::new(aligned_address)
                }
            }
            #[inline]
            pub fn align_up(self, alignment: u64) -> Self {
                Self::new(self.inner + (alignment - 1)).align_down(alignment)
            }
        }
    };
}
