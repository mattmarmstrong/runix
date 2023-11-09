use conquer_once::spin::OnceCell;

use crate::acpi::ACPI_TABLES;
use crate::cpu::CPU_INFO;
use crate::mmu::{
    phys_to_virt_address,
    PhysicalAddress,
    VirtualAddress,
};
use crate::util::volatile::Volatile;

pub static IOAPIC: OnceCell<IOAPIC> = OnceCell::uninit();

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct IOAPIC {
    address: VirtualAddress,
}

impl IOAPIC {
    pub fn read_register(&self, register_offset: u32) -> u32 {
        let register_address = VirtualAddress::new(self.address.inner + register_offset as u64);
        Volatile::new(register_address.inner).read() as u32
    }

    pub fn write_to_register(&self, register_offset: u32, value: u32) {
        let register_address = VirtualAddress::new(self.address.inner + register_offset as u64);
        Volatile::new(register_address.inner).write(value as u64)
    }
}

pub fn init_ioapic_from_acpi() {}
