use crate::mmu::vmm::frame::PhysicalFrame;
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
    pub const PRESENT: u64 = 1;
    pub const WRITE_ACCESS: u64 = 1 << 1;
    pub const USER_ACCESS: u64 = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const CACHE_DISABLED: u64 = 1 << 4;
    pub const ACCESSED: u64 = 1 << 5;
    // Shoutout to the Black-eyed Peas
    pub const DIRTY: u64 = 1 << 6;
    pub const LARGE_PAGE_SIZE: u64 = 1 << 7;
    pub const GLOBAL: u64 = 1 << 8;
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry {
    inner: u64,
}

impl PageTableEntry {
    pub fn new(flags: u64, physical_address: PhysicalAddress) -> Self {
        PageTableEntry {
            inner: (flags & physical_address.inner),
        }
    }

    #[inline]
    pub fn new_unused() -> Self {
        PageTableEntry { inner: 0 }
    }

    #[inline]
    pub fn is_unused(&self) -> bool {
        self.inner == 0
    }

    pub fn set_unused(&mut self) {
        self.inner = 0
    }

    #[inline]
    pub fn flags(&self) -> u64 {
        self.inner & 0xFF
    }

    #[inline]
    pub fn is_flag_set(&self, entry_flag: u64) -> bool {
        is_bit_set(self.inner, entry_flag)
    }
    #[inline]
    pub fn set_flag(&mut self, entry_flag: u64) {
        self.inner = set_bit(self.inner, entry_flag)
    }

    #[inline]
    pub fn set_flags(&mut self, entry_flags: u64) {
        self.inner |= entry_flags
    }

    #[inline]
    fn get_physical_address(&self) -> PhysicalAddress {
        PhysicalAddress::new(self.inner & PHYSICAL_ADDRESS_MASK)
    }

    pub fn get_physical_frame(&self) -> Option<PhysicalFrame> {
        match self.is_flag_set(PageTableEntryFlags::PRESENT) {
            true => Some(PhysicalFrame::from_address_aligned(self.get_physical_address())),
            false => None,
        }
    }

    pub fn set_physical_frame_address(&mut self, physical_frame: PhysicalFrame) {
        self.inner &= physical_frame.start_address()
    }
}
