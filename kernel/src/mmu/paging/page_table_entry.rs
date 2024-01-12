use crate::mmu::paging::frame::PhysicalFrame;
use crate::mmu::paging::page::PageSize;
use crate::mmu::PhysicalAddress;
use crate::util::bits::{
    is_bit_set,
    set_bit,
};

const PHYSICAL_ADDRESS_MASK: u64 = 0x000F_FFFF_FFFF_F000;

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
    // Shoutout to the Black-eyed Peas
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

    // this is kind of dumb lmao
    pub fn is_flag_set(&self, entry_flag: u64) -> bool {
        is_bit_set(self.inner, entry_flag)
    }

    pub fn set_flag(&mut self, entry_flag: u64) {
        self.inner = set_bit(self.inner, entry_flag)
    }

    // Default bit-setters
    pub fn set_present(&mut self) {
        self.inner = set_bit(self.inner, PageTableEntryFlags::PRESENT)
    }

    pub fn set_write_access(&mut self) {
        self.inner = set_bit(self.inner, PageTableEntryFlags::WRITE_ACCESS)
    }

    fn get_physical_address(&self) -> PhysicalAddress {
        PhysicalAddress::new(self.inner & PHYSICAL_ADDRESS_MASK)
    }

    pub fn get_physical_frame(&self) -> Option<PhysicalFrame> {
        match self.is_flag_set(PageTableEntryFlags::PRESENT) {
            true => Some(PhysicalFrame::from_address_aligned(
                self.get_physical_address(),
                PageSize::FOUR_KIB,
            )),
            false => None,
        }
    }
}
