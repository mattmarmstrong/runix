use core::arch::asm;

use crate::mmu::paging::page_table_entry::PageTableEntry;

const PAGE_TABLE_ENTRIES: usize = 512;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageMapLevel4 {
    table: [PageTableEntry; PAGE_TABLE_ENTRIES],
}

impl PageMapLevel4 {
    // We load the pml4 that's mapped by the bootloader currently, so we should just load it from
    // the address in the CR3 register
    pub unsafe fn get_active_pml4() -> &'static mut Self {
        let raw_pml4_address: u64;
        asm!("mov {}, cr3", out(reg) raw_pml4_address, options(nomem, nostack, preserves_flags));
        let table_ptr = raw_pml4_address as *mut PageMapLevel4;
        &mut *table_ptr
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageDirectoryPointerTable {
    table: [PageTableEntry; PAGE_TABLE_ENTRIES],
}

#[derive(Debug, Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageDirectoryTable {
    table: [PageTableEntry; PAGE_TABLE_ENTRIES],
}

#[derive(Debug, Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageTable {
    table: [PageTableEntry; PAGE_TABLE_ENTRIES],
}
