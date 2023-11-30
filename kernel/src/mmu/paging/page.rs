use crate::mmu::VirtualAddress;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum PageSize {}

impl PageSize {
    pub const FOUR_KIB: u64 = 1 << 12;
    pub const TWO_MB: u64 = (1 << 20) * 2;
    pub const ONE_GB: u64 = 1 << 30;
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Page {
    size: u64,
    offset: VirtualAddress,
}

impl Page {
    pub fn from_address_aligned(virtual_address: VirtualAddress, page_size: u64) -> Self {
        let aligned_address = virtual_address.align_down(page_size);
        Page {
            size: page_size,
            offset: aligned_address,
        }
    }
}
