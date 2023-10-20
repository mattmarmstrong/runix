use crate::mmu::paging::page_table::PageTableEntry;
use crate::mmu::paging::PageSize;
use crate::mmu::VirtualAddress;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Page {
    size: PageSize,
    offset: VirtualAddress,
    pte: PageTableEntry,
}

impl Page {
    pub fn from_address_aligned(virtual_address: VirtualAddress, page_size: PageSize) -> Self {
        let aligned_address = virtual_address.align_down(page_size as u64);
        Page {
            size: page_size,
            offset: aligned_address,
            pte: PageTableEntry::new(0),
        }
    }
}
