// memory management software

pub mod paging;
pub mod tlb;

pub const KERNEL_BASE_ADDRESS: u64 = 0xFFFF_8880_0000_0000;

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

impl core::ops::Sub for VirtualAddress {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let raw_sub_result = self.inner + rhs.inner;
        VirtualAddress::new(raw_sub_result)
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

    #[inline]
    pub fn get_page_offset(self) -> u16 {
        (self.inner & 0x0FFF) as u16
    }

    // we have to use a usize to index into the page table, so we might as well do the necessary conversions here
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddress {
    pub inner: u64,
}

#[derive(Debug)]
pub struct InvalidPhysicalAddress {
    pub inner: u64,
}

impl core::ops::Add for PhysicalAddress {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let raw_add_result = self.inner + rhs.inner;
        PhysicalAddress::new(raw_add_result)
    }
}

impl core::ops::Sub for PhysicalAddress {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let raw_sub_result = self.inner + rhs.inner;
        PhysicalAddress::new(raw_sub_result)
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
    pub fn new(address: u64) -> Self {
        PhysicalAddress::try_new(address).unwrap()
    }

    #[inline]
    fn try_new(address: u64) -> Result<Self, InvalidPhysicalAddress> {
        match address >> 52 {
            0 => Ok(PhysicalAddress { inner: address }),
            _ => Err(InvalidPhysicalAddress { inner: address }),
        }
    }
}

// There was absolutely no need to do this. I just wanted to write a macro
macro_rules! impl_alignment_functions {
    ($addr_type: ty) => {
        impl $addr_type {
            #[inline]
            pub fn align_down(self, alignment: u64) -> Self {
                debug_assert!(alignment.is_power_of_two());
                if self.inner % (alignment) == 0 {
                    self
                } else {
                    let alignment_mask: u64 = !(alignment - 1);
                    let aligned_address: u64 = self.inner & alignment_mask;
                    Self::new(aligned_address)
                }
            }
            #[inline]
            pub fn align_up(self, alignment: u64) -> Self {
                Self::new(self.inner + (alignment - 1)).align_down(alignment)
            }
        }
    };
}

impl_alignment_functions!(VirtualAddress);
impl_alignment_functions!(PhysicalAddress);

pub fn phys_to_virt_address(phys_address: PhysicalAddress) -> VirtualAddress {
    VirtualAddress::new(KERNEL_BASE_ADDRESS + phys_address.inner)
}

pub fn virt_to_phys_address(virt_address: VirtualAddress) -> PhysicalAddress {
    PhysicalAddress::new(virt_address.inner - KERNEL_BASE_ADDRESS)
}
