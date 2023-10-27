use core::mem::size_of;

use crate::acpi::sdt::{
    SDTHeader,
    SDTSignature,
    SystemDescriptorTable,
};
use crate::mmu::{
    phys_to_virt_address,
    PhysicalAddress,
};

// I'm cheating. The machine being emulated by the virtualization software I'm using for
// development supports ACPI Revision 2.0. The structure of this data reflects that
// https://uefi.org/sites/default/files/resources/ACPI_2.pdf

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct APICStructureHeader {
    entry_type: u8,
    length: u8,
}

impl APICStructureHeader {}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct ProcessorLocalAPIC {
    apic_processor_id: u8,
    apic_id: u8,
    lapic_id: u8,
    flags: u32,
}

impl ProcessorLocalAPIC {}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IOAPIC {
    io_apic_id: u8,
    _reserved: u8,
    io_apic_physical_address: u32,
    global_system_interrupt_base: u32,
}

impl IOAPIC {}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct InterruptSourceOverride {
    bus: u8,
    source: u8,
    global_system_interrupt: u32,
    mps_inti_flags: u16,
}

impl IOAPIC {}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MADTHeader {
    lapic_address: u32,
    multiple_apic_flags: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MADT {
    header: SDTHeader,
    madt_header: MADTHeader,
}

impl SystemDescriptorTable for MADT {
    unsafe fn init(raw_madt_physical_address: u64) -> Self {
        let header = SDTHeader::try_read_from_phys_addr(raw_madt_physical_address, &SDTSignature::MADT).unwrap();
        let size_of_sdt_header = size_of::<SDTHeader>() as u64;
        let madt_header_phys_addr = PhysicalAddress::new(raw_madt_physical_address + size_of_sdt_header);
        let madt_header_virt_addr = phys_to_virt_address(madt_header_phys_addr);
        let raw_madt_header = madt_header_virt_addr.inner as *const MADTHeader;
        let madt_header = *raw_madt_header;
        MADT { header, madt_header }
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct APICStructures {
    processor_local_apic: ProcessorLocalAPIC,
    io_apic: IOAPIC,
    interrupt_source_override: InterruptSourceOverride,
}

impl APICStructures {
    pub unsafe fn parse_and_return_apic_structs(raw_madt_physical_address: u64) -> APICStructures {
        let madt = MADT::init(raw_madt_physical_address);
        let madt_address =  
    }
}
