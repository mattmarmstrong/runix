use core::ptr;

// Very crude implementation of a 'volatile' value
// See: https://barrgroup.com/embedded-systems/how-to/c-volatile-keyword

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Volatile<InnerType: Copy> {
    inner: InnerType,
}

impl<InnerType: Copy> Volatile<InnerType> {
    pub fn new(inner: InnerType) -> Volatile<InnerType> {
        Volatile { inner }
    }

    pub fn read(&self) -> InnerType {
        unsafe { ptr::read_volatile(&self.inner) }
    }

    pub fn write(&mut self, value: InnerType) {
        unsafe { ptr::write_volatile(&mut self.inner, value) }
    }
}
