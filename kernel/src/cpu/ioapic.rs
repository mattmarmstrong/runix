use conquer_once::spin::OnceCell;

use crate::mmu::address::VirtualAddress;

pub static IOAPIC: OnceCell<IOAPIC> = OnceCell::uninit();

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct IOAPIC {
    address: VirtualAddress,
}

impl IOAPIC {
    pub fn read_register(&self, register_offset: u32) -> u32 {
        let register_address = VirtualAddress::with_offset(self.address.inner, register_offset as usize);
        unsafe { core::ptr::read_volatile(register_address.inner as *const u32) }
    }

    pub fn write_to_register(&self, register_offset: u32, value: u32) {
        let register_address = VirtualAddress::with_offset(self.address.inner, register_offset as usize);
        unsafe { core::ptr::write_volatile(register_address.inner as *mut u32, value) }
    }
}

pub fn init_ioapic_from_acpi() {}
