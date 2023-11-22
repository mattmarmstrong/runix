use crate::acpi::ACPI_TABLES;
use crate::cpu::msr::{
    read_msr_value,
    IA32_APIC_MSR_BASE,
};
use crate::cpu::pit::{
    PIT,
    PIT_FREQUENCY,
};
use crate::cpu::CPU_INFO;
use crate::interrupts::InterruptVector;
use crate::mmu::{
    phys_to_virt_address,
    PhysicalAddress,
    VirtualAddress,
};

// TODO: Check if there is an MSR, Read the MSR value
const IA32_APIC_BASE_MSR: u64 = 0x1B;

// If this bit is set in a LAPIC register, the corresponding interrupt is masked
const LAPIC_INTERRUPT_MASK: u32 = 1 << 16;
const LAPIC_TIMER_MODE_PERIODIC: u32 = 1 << 17;

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
    virtual_address: VirtualAddress,
}

impl LocalAPIC {
    pub fn read_register(&self, register_offset: u32) -> u32 {
        let register_address = VirtualAddress::new(self.virtual_address.inner + (register_offset as u64));
        unsafe { core::ptr::read_volatile(register_address.inner as *const u32) }
    }

    pub fn write_to_register(&self, register_offset: u32, value: u32) {
        let register_address = VirtualAddress::new(self.virtual_address.inner + (register_offset as u64));
        unsafe { core::ptr::write_volatile(register_address.inner as *mut u32, value) }
    }

    pub fn read_id(&self) -> u32 {
        self.read_register(LAPICRegister::LAPIC_ID) << 24
    }

    pub fn clear_task_priority_register(&self) {
        self.write_to_register(LAPICRegister::TASK_PRIORITY, 0x00)
    }

    pub fn enable_interrupts(&self) {
        // See Chapter 10 Section 4.3 of the Intel manual
        // Spurious interrupts are mapped to IRQ 0xFF in the IDT
        self.write_to_register(
            LAPICRegister::SPURIOUS_INTERRUPT_VECTOR,
            (0x100 | InterruptVector::APIC_SPURIOUS) as u32,
        );
    }

    pub fn signal_end_of_interrupt(&self) {
        self.write_to_register(LAPICRegister::END_OF_INTERRUPT, 0);
    }

    pub fn stop_timer(&self) {
        // reset the initial timer count
        self.write_to_register(LAPICRegister::TIMER_INITIAL_COUNT, 0x00);
        // mask timer interrupts
        self.write_to_register(LAPICRegister::TIMER_LOCAL_VECTOR_TABLE_ENTRY, LAPIC_INTERRUPT_MASK);
    }

    pub fn calibrate_and_init_periodic_timer(&self) {
        // Find the LAPIC frequency, which apparently corresponds to the core clock frequency
        // https://lkml.iu.edu/hypermail/linux/kernel/1904.2/04624.html
        let lapic_tick_samples: u32 = 0xFFFFF;
        let pit = PIT::new();

        self.write_to_register(LAPICRegister::TIMER_INITIAL_COUNT, lapic_tick_samples);

        pit.set_count(0xFFFF);
        let pit_intial_tick_count = pit.read_count();

        self.write_to_register(LAPICRegister::TIMER_INITIAL_COUNT, lapic_tick_samples);

        let pit_final_tick_count = pit.read_count();
        let total_pit_ticks = pit_intial_tick_count - pit_final_tick_count;

        let lapic_timer_frequency = (lapic_tick_samples - total_pit_ticks as u32) * (PIT_FREQUENCY as u32);

        let timer_micro_seconds = 5000;

        let lapic_interrupt_timer_ticks = timer_micro_seconds * (lapic_timer_frequency / 1_000_000);

        self.write_to_register(
            LAPICRegister::TIMER_LOCAL_VECTOR_TABLE_ENTRY,
            InterruptVector::APIC_TIMER as u32 | LAPIC_TIMER_MODE_PERIODIC,
        );
        self.write_to_register(LAPICRegister::TIMER_DIVIDE_CONFIGURATION, 0x01);
        self.write_to_register(LAPICRegister::TIMER_INITIAL_COUNT, lapic_interrupt_timer_ticks);
        log::info!("LAPIC Timer calibrated!");
    }

    pub fn try_read_and_init_from_madt() -> Option<Self> {
        let cpu_info = CPU_INFO.get().unwrap();
        if cpu_info.apic_enabled {
            let physical_address_base: PhysicalAddress;
            match cpu_info.msr_present {
                true => {
                    let apic_msr_read_value = unsafe { read_msr_value(IA32_APIC_MSR_BASE) };
                    physical_address_base = PhysicalAddress::new(apic_msr_read_value);
                }
                false => {
                    let apic_structures = &ACPI_TABLES.get().unwrap().madt.apic_structures;
                    let apic_headers = &ACPI_TABLES.get().unwrap().madt.apic_headers;
                    match apic_structures.local_apic_address_override {
                        Some(lapic_address_override_record) => {
                            physical_address_base =
                                PhysicalAddress::new(lapic_address_override_record.local_apic_address_64)
                        }
                        None => {
                            physical_address_base = PhysicalAddress::new(apic_headers.madt_header.lapic_address as u64)
                        }
                    }
                }
            }
            let virtual_address = phys_to_virt_address(physical_address_base);
            Some(LocalAPIC { virtual_address })
        } else {
            log::error!("APIC not supported by CPU!");
            None
        }
    }

    pub fn initialize_core_lapic() -> Self {
        let lapic = Self::try_read_and_init_from_madt().unwrap();
        log::info!("LAPIC detected at address: {:#X}", lapic.virtual_address.inner);
        unsafe { core::arch::asm!("cli", options(nomem, nostack)) }
        lapic.clear_task_priority_register();
        lapic.enable_interrupts();
        log::info!("LAPIC interrupts enabled");
        lapic.calibrate_and_init_periodic_timer();
        log::info!("LAPIC Timer calibrated and initialized");
        unsafe { core::arch::asm!("sti", options(nomem, nostack)) }
        lapic
    }
}
