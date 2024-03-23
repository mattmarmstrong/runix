use crate::mmu::KERNEL_BASE_ADDRESS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualAddress {
    pub inner: usize,
}

#[derive(Debug)]
pub struct InvalidVirtualAddress {
    pub inner: usize,
}

impl core::ops::Add for VirtualAddress {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let raw_add_result = self.inner + rhs.inner;
        VirtualAddress::new(raw_add_result)
    }
}

impl core::ops::Add<usize> for VirtualAddress {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
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

impl core::ops::Sub<usize> for VirtualAddress {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
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
    pub fn new(address: usize) -> Self {
        VirtualAddress::try_new(address).unwrap()
    }

    #[inline]
    fn try_new(address: usize) -> Result<Self, InvalidVirtualAddress> {
        match address >> 48 {
            0 | 0xFFFF => Ok(VirtualAddress { inner: address }),
            // sign extension
            1 => Ok(VirtualAddress {
                inner: (((address << 16) as i64 >> 16) as usize),
            }),
            _ => Err(InvalidVirtualAddress { inner: address }),
        }
    }

    #[inline]
    pub fn with_offset(address: usize, offset: usize) -> Self {
        VirtualAddress::new(address + offset)
    }

    #[inline]
    pub fn with_kernel_base_offset(address: usize) -> Self {
        VirtualAddress::with_offset(address, KERNEL_BASE_ADDRESS)
    }

    #[inline]
    pub const fn kernel_base() -> Self {
        VirtualAddress {
            inner: KERNEL_BASE_ADDRESS,
        }
    }

    #[inline]
    pub fn get_page_offset(self) -> usize {
        ((self.inner & 0x0FFF) as u16) as usize
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

    #[inline]
    pub fn is_aligned(&self) -> bool {
        self.inner % 4096 == 0
    }

    #[inline]
    pub fn add_checked(&self, rhs: usize) -> Self {
        let new_inner = self.inner.checked_add(rhs).unwrap();
        VirtualAddress::new(new_inner)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress {
    pub inner: usize,
}

#[derive(Debug)]
pub struct InvalidPhysicalAddress {
    pub inner: usize,
}

impl core::ops::Add for PhysicalAddress {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let raw_add_result = self.inner + rhs.inner;
        PhysicalAddress::new(raw_add_result)
    }
}

impl core::ops::Add<usize> for PhysicalAddress {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        PhysicalAddress::new(self.inner + rhs)
    }
}

impl core::ops::Sub for PhysicalAddress {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let raw_sub_result = self.inner + rhs.inner;
        PhysicalAddress::new(raw_sub_result)
    }
}

impl core::ops::Sub<usize> for PhysicalAddress {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        PhysicalAddress::new(self.inner - rhs)
    }
}

impl core::fmt::Display for PhysicalAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("Physical Address: {:#X}", self.inner))
    }
}

impl PhysicalAddress {
    #[inline]
    pub const fn zero() -> Self {
        PhysicalAddress { inner: 0 }
    }

    #[inline]
    pub fn new(address: usize) -> Self {
        PhysicalAddress::try_new(address).unwrap()
    }

    #[inline]
    fn try_new(address: usize) -> Result<Self, InvalidPhysicalAddress> {
        match address >> 52 {
            0 => Ok(PhysicalAddress { inner: address }),
            _ => Err(InvalidPhysicalAddress { inner: address }),
        }
    }
}

// There was absolutely no need to do this. I just wanted to write a macro
#[macro_export]
macro_rules! impl_alignment_functions {
    ($addr_type: ty) => {
        impl $addr_type {
            #[inline]
            pub fn align_down(self, alignment: usize) -> Self {
                debug_assert!(alignment.is_power_of_two());
                if self.inner % (alignment) == 0 {
                    self
                } else {
                    let alignment_mask: usize = !(alignment - 1);
                    let aligned_address: usize = self.inner & alignment_mask;
                    Self::new(aligned_address)
                }
            }
            #[inline]
            pub fn align_up(self, alignment: usize) -> Self {
                Self::new(self.inner + (alignment - 1)).align_down(alignment)
            }
        }
    };
}

impl_alignment_functions!(VirtualAddress);
impl_alignment_functions!(PhysicalAddress);
