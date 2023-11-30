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
