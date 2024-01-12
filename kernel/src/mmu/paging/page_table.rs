use crate::mmu::paging::asm::get_raw_pml4_ptr;
use crate::mmu::paging::page_table_entry::PageTableEntry;
use crate::mmu::virtual_address::VirtualAddress;

const PAGE_TABLE_ENTRIES: usize = 512;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageTable {
    table: [PageTableEntry; PAGE_TABLE_ENTRIES],
}

impl PageTable {
    pub unsafe fn get_active_pml4() -> &'static mut Self {
        let raw_pml4_address = get_raw_pml4_ptr();
        let table_ptr = raw_pml4_address as *mut PageTable;
        &mut *table_ptr
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct MappedPageTable {
    page_table: PageTable,
    offset: VirtualAddress,
}

impl MappedPageTable {
    pub fn get_next_page_table(&self, page_table_entry: PageTableEntry) -> PageTable {
        page_table_entry
            .get_physical_frame()
            .unwrap()
            .frame_to_page_table(self.offset)
    }
}
