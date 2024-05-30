use crate::mmu::address::PhysicalAddress;
use crate::mmu::vmm::frame::PhysicalFrame;
use crate::util::bits::{
    is_bit_set,
    set_bit,
};

pub const PHYSICAL_ADDRESS_MASK: usize = 0x000F_FFFF_FFFF_F000;

// Entry flags
#[non_exhaustive]
pub enum PageTableEntryFlags {}

impl PageTableEntryFlags {
    pub const PRESENT: usize = 1;
    pub const WRITE_ACCESS: usize = 1 << 1;
    pub const USER_ACCESS: usize = 1 << 2;
    pub const WRITE_THROUGH: usize = 1 << 3;
    pub const CACHE_DISABLED: usize = 1 << 4;
    pub const ACCESSED: usize = 1 << 5;
    // Shoutout to the Black-eyed Peas
    pub const DIRTY: usize = 1 << 6;
    pub const LARGE_PAGE_SIZE: usize = 1 << 7;
    pub const GLOBAL: usize = 1 << 8;
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry {
    pub inner: usize,
}

impl PageTableEntry {
    pub fn new(flags: usize, physical_address: PhysicalAddress) -> Self {
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
    pub fn flags(&self) -> usize {
        self.inner & 0xFF
    }

    #[inline]
    pub fn is_flag_set(&self, entry_flag: usize) -> bool {
        is_bit_set(self.inner, entry_flag)
    }
    #[inline]
    pub fn set_flag(&mut self, entry_flag: usize) {
        self.inner = set_bit(self.inner, entry_flag)
    }

    #[inline]
    pub fn set_flags(&mut self, entry_flags: usize) {
        self.inner |= entry_flags
    }

    #[inline]
    fn get_physical_addr(&self) -> PhysicalAddress {
        PhysicalAddress::new(self.inner & PHYSICAL_ADDRESS_MASK)
    }

    pub fn get_frame(&self) -> Option<PhysicalFrame> {
        match self.is_flag_set(PageTableEntryFlags::PRESENT) {
            true => Some(PhysicalFrame::from_address_aligned(self.get_physical_addr())),
            false => None,
        }
    }

    pub fn set_frame_addr(&mut self, physical_frame: PhysicalFrame) {
        self.inner &= physical_frame.start_address()
    }
}
