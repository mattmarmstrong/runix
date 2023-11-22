use conquer_once::spin::OnceCell;

use crate::acpi::ACPI_TABLES;
use crate::cpu::CPU_INFO;
use crate::mmu::{
    phys_to_virt_address,
    PhysicalAddress,
    VirtualAddress,
};

pub static IOAPIC: OnceCell<IOAPIC> = OnceCell::uninit();

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct IOAPIC {
    address: VirtualAddress,
}

impl IOAPIC {
    pub fn read_register(&self, register_offset: u32) -> u32 {
        let register_address = VirtualAddress::new(self.address.inner + (register_offset as u64));
        unsafe { core::ptr::read_volatile(register_address.inner as *const u32) }
    }

    pub fn write_to_register(&self, register_offset: u32, value: u32) {
        let register_address = VirtualAddress::new(self.address.inner + (register_offset as u64));
        unsafe { core::ptr::write_volatile(register_address.inner as *mut u32, value) }
    }
}

pub fn init_ioapic_from_acpi() {}
