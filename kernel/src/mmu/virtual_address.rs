use core::u64;

use crate::impl_alignment_functions;
use crate::mmu::KERNEL_BASE_ADDRESS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualAddress {
    pub inner: u64,
}

#[derive(Debug)]
pub struct InvalidVirtualAddress {
    pub inner: u64,
}

impl core::ops::Add for VirtualAddress {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let raw_add_result = self.inner + rhs.inner;
        VirtualAddress::new(raw_add_result)
    }
}

impl core::ops::Add<u64> for VirtualAddress {
    type Output = Self;
    fn add(self, rhs: u64) -> Self::Output {
        VirtualAddress::new(self.inner + rhs)
    }
}

impl core::ops::Sub for VirtualAddress {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let raw_sub_result = self.inner + rhs.inner;
        VirtualAddress::new(raw_sub_result)
    }
}

impl core::ops::Sub<u64> for VirtualAddress {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self::Output {
        VirtualAddress::new(self.inner - rhs)
    }
}

impl core::fmt::Display for VirtualAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("Virtual Address: {:#X}", self.inner))
    }
}

impl VirtualAddress {
    #[inline]
    pub const fn zero() -> Self {
        VirtualAddress { inner: 0 }
    }

    #[inline]
    pub fn new(address: u64) -> Self {
        VirtualAddress::try_new(address).unwrap()
    }

    #[inline]
    fn try_new(address: u64) -> Result<Self, InvalidVirtualAddress> {
        match address >> 48 {
            0 | 0xFFFF => Ok(VirtualAddress { inner: address }),
            // sign extension
            1 => Ok(VirtualAddress {
                inner: (((address << 16) as i64 >> 16) as u64),
            }),
            _ => Err(InvalidVirtualAddress { inner: address }),
        }
    }

    pub fn with_offset(address: u64, offset: u64) -> Self {
        VirtualAddress::new(address + offset)
    }

    pub fn with_kernel_base_offset(address: u64) -> Self {
        VirtualAddress::with_offset(address, KERNEL_BASE_ADDRESS)
    }

    #[inline]
    pub fn get_page_offset(self) -> u64 {
        ((self.inner & 0x0FFF) as u16) as u64
    }

    #[inline]
    pub fn get_pt_index(self) -> usize {
        ((self.inner >> 12) & 0x01FF) as usize
    }

    #[inline]
    pub fn get_pd_index(self) -> usize {
        ((self.inner >> 21) & 0x01FF) as usize
    }

    #[inline]
    pub fn get_pdpt_index(self) -> usize {
        ((self.inner >> 30) & 0x01FF) as usize
    }

    #[inline]
    pub fn get_pml4_index(self) -> usize {
        ((self.inner >> 39) & 0x01FF) as usize
    }
}

impl_alignment_functions!(VirtualAddress);
