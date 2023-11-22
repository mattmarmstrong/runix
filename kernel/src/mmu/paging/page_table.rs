use core::arch::asm;

use crate::mmu::paging::PHYSICAL_ADDRESS_MASK;
use crate::mmu::{
    PhysicalAddress,
    VirtualAddress,
};

const PAGE_TABLE_ENTRIES: usize = 512;

// Entry flags
#[non_exhaustive]
pub enum PageTableEntryFlags {}

impl PageTableEntryFlags {
    const PRESENT: u64 = 1;
    const WRITE_ACCESS: u64 = 1 << 1;
    const USER_ACCESS: u64 = 1 << 2;
    const WRITE_THROUGH: u64 = 1 << 3;
    const CACHE_DISABLED: u64 = 1 << 4;
    const ACCESSED: u64 = 1 << 5;
    const DIRTY: u64 = 1 << 6;
    const LARGE_PAGE_SIZE: u64 = 1 << 7;
    const GLOBAL: u64 = 1 << 8;
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry {
    inner: u64,
}

impl PageTableEntry {
    pub fn new(flags: u64) -> Self {
        PageTableEntry { inner: flags }
    }

    // flag checking convience methods
    pub fn is_page_present(&self) -> bool {
        (self.inner & PageTableEntryFlags::PRESENT) == PageTableEntryFlags::PRESENT
    }

    pub fn allows_write_access(&self) -> bool {
        (self.inner & PageTableEntryFlags::WRITE_ACCESS) == PageTableEntryFlags::WRITE_ACCESS
    }
    pub fn allows_user_access(&self) -> bool {
        (self.inner & PageTableEntryFlags::USER_ACCESS) == PageTableEntryFlags::USER_ACCESS
    }
    pub fn can_write_through(&self) -> bool {
        (self.inner & PageTableEntryFlags::WRITE_THROUGH) == PageTableEntryFlags::WRITE_THROUGH
    }

    pub fn cache_disabled(&self) -> bool {
        (self.inner & PageTableEntryFlags::CACHE_DISABLED) == PageTableEntryFlags::CACHE_DISABLED
    }

    pub fn has_been_accessed(&self) -> bool {
        (self.inner & PageTableEntryFlags::ACCESSED) == PageTableEntryFlags::ACCESSED
    }

    pub fn is_dirty(&self) -> bool {
        (self.inner & PageTableEntryFlags::DIRTY) == PageTableEntryFlags::DIRTY
    }

    pub fn is_large_page(&self) -> bool {
        (self.inner & PageTableEntryFlags::LARGE_PAGE_SIZE) == PageTableEntryFlags::LARGE_PAGE_SIZE
    }

    pub fn is_global(&self) -> bool {
        (self.inner & PageTableEntryFlags::GLOBAL) == PageTableEntryFlags::GLOBAL
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageMapLevel4 {
    table: [PageTableEntry; PAGE_TABLE_ENTRIES],
}

impl PageMapLevel4 {
    // We load the pml4 that's mapped by the bootloader currently, so we should just load it from
    // the address in the CR3 register
    pub unsafe fn get_active_pml4(pml4_virtual_address: VirtualAddress) -> &'static mut Self {
        let raw: u64;
        asm!("mov {}, cr3", out(reg) raw, options(nomem, nostack, preserves_flags));
        let table_ptr = pml4_virtual_address.inner as *mut PageMapLevel4;
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
