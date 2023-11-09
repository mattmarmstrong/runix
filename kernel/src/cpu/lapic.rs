use core::u32;

use crate::acpi::ACPI_TABLES;
use crate::cpu::CPU_INFO;
use crate::mmu::{
    phys_to_virt_address,
    PhysicalAddress,
    VirtualAddress,
};
use crate::util::volatile::Volatile;

// If this bit is set in a LAPIC register, the corresponding interrupt is masked
pub const LAPIC_INTERRUPT_MASK: u32 = 1 << 16;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum LAPICRegister {}

impl LAPICRegister {
    // These represent address offsets to each individual register from the LAPIC_ADDRESS. They're all memory-mapped
    // See Chapter 10:
    // https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-3a-part-1-manual.pdf
    pub const LAPIC_ID: u32 = 0x020;
    pub const LAPIC_VERSION: u32 = 0x030;
    pub const TASK_PRIORITY: u32 = 0x080;
    pub const END_OF_INTERRUPT: u32 = 0x0B0;
    pub const SPURIOUS_INTERRUPT_VECTOR: u32 = 0x0F0;
    pub const TIMER_LOCAL_VECTOR_TABLE_ENTRY: u32 = 0x320;
    pub const ERROR_LOCAL_VECTOR_TABLE_ENTRY: u32 = 0x370;
    pub const TIMER_INITIAL_COUNT: u32 = 0x380;
    pub const TIMER_CURRENT_COUNT: u32 = 0x390;
    pub const TIMER_DIVIDE_CONFIGURATION: u32 = 0x3E0;
}

#[derive(Debug, Clone, Copy)]
pub struct LocalAPIC {
    lapic_id: usize,
    address: VirtualAddress,
}

impl LocalAPIC {
    pub fn read_register(&self, register_offset: u32) -> u32 {
        let register_address = VirtualAddress::new(self.address.inner + (register_offset as u64));
        Volatile::new(register_address.inner).read() as u32
    }

    pub fn write_to_register(&self, register_offset: u32, value: u32) {
        let register_address = VirtualAddress::new(self.address.inner + (register_offset as u64));
        Volatile::new(register_address.inner).write(value as u64)
    }

    pub fn clear_task_priority_register(&self) {
        self.write_to_register(LAPICRegister::TASK_PRIORITY, 0x00)
    }

    pub fn enable_interrupts(&self) {
        // See Chapter 10 Section 4.3 of the Intel manual
        // Spurious interrupts are mapped to IRQ 0xFF in the IDT
        self.write_to_register(LAPICRegister::SPURIOUS_INTERRUPT_VECTOR, 0x1FF)
    }

    pub fn signal_end_of_interrupt(&self) {
        self.write_to_register(LAPICRegister::END_OF_INTERRUPT, 0)
    }

    pub fn stop_timer(&self) {
        // reset the initial timer count
        self.write_to_register(LAPICRegister::TIMER_INITIAL_COUNT, 0x00);
        // mask timer interrupts
        self.write_to_register(LAPICRegister::TIMER_LOCAL_VECTOR_TABLE_ENTRY, LAPIC_INTERRUPT_MASK);
    }

    pub fn calibrate_timer(&self) {
        const SAMPLE: u32 = u32::MAX;

        // NEED TO USE THE PIT TO CALIBRATE THE LAPIC TIMER
    }

    pub fn try_read_and_init_from_madt(lapic_id: usize) -> Option<Self> {
        if CPU_INFO.apic_enabled {
            let apic_structures = &ACPI_TABLES.get().unwrap().madt.apic_structures;
            let apic_headers = &ACPI_TABLES.get().unwrap().madt.apic_headers;
            let lapic_physical_address: PhysicalAddress;
            match apic_structures.local_apic_address_override {
                Some(lapic_address_override_record) => {
                    lapic_physical_address = PhysicalAddress::new(lapic_address_override_record.local_apic_address_64)
                }
                None => lapic_physical_address = PhysicalAddress::new(apic_headers.madt_header.lapic_address as u64),
            }
            let address = phys_to_virt_address(lapic_physical_address);
            Some(LocalAPIC { address, lapic_id })
        } else {
            log::error!("APIC not supported by CPU!");
            None
        }
    }

    pub fn initialize_core_lapic(lapic_id: usize) -> Self {
        let lapic = Self::try_read_and_init_from_madt(lapic_id).unwrap();
        lapic.clear_task_priority_register();
        lapic.enable_interrupts();
    }
}
