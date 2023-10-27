use core::mem::size_of;

use crate::acpi::sdt::{
    SDTHeader,
    SDTSignature,
    SystemDescriptorTable,
};
use crate::mmu::{
    phys_to_virt_address,
    PhysicalAddress,
    VirtualAddress,
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
    apic_struct_header: APICStructureHeader,
    apic_processor_id: u8,
    apic_id: u8,
    lapic_id: u8,
    flags: u32,
}

impl ProcessorLocalAPIC {}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IOAPIC {
    apic_struct_header: APICStructureHeader,
    io_apic_id: u8,
    _reserved: u8,
    pub io_apic_physical_address: u32,
    global_system_interrupt_base: u32,
}

impl IOAPIC {}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct InterruptSourceOverride {
    apic_struct_header: APICStructureHeader,
    bus: u8,
    source: u8,
    global_system_interrupt: u32,
    mps_inti_flags: u16,
}

impl IOAPIC {}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct MADTHeader {
    lapic_address: u32,
    multiple_apic_flags: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct MADT {
    // This doesn't represent the entirety of the MADT. These are the headers that we can parse
    // to read the data about the CPUs APIC Structures that exist adjacent in memory
    header: SDTHeader,
    madt_header: MADTHeader,
}

impl SystemDescriptorTable for MADT {
    unsafe fn init(raw_madt_physical_address: u64) -> Self {
        let header = SDTHeader::try_read_from_phys_addr(raw_madt_physical_address, &SDTSignature::MADT).unwrap();
        let size_of_sdt_header = size_of::<SDTHeader>() as u64;
        let madt_header_phys_addr = PhysicalAddress::new(raw_madt_physical_address + size_of_sdt_header);
        let madt_header_virt_addr = phys_to_virt_address(madt_header_phys_addr);
        let madt_header_ref = madt_header_virt_addr.inner as *const MADTHeader;
        let madt_header = *madt_header_ref;
        MADT { header, madt_header }
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct APICStructures {
    pub processor_local_apic: ProcessorLocalAPIC,
    pub io_apic: IOAPIC,
    pub interrupt_source_override: InterruptSourceOverride,
}

impl APICStructures {
    pub unsafe fn try_parse_apic_structs(raw_madt_physical_address: u64) -> Option<APICStructures> {
        let madt = MADT::init(raw_madt_physical_address);
        let madt_address = VirtualAddress::new(&madt as *const _ as u64);

        // The addresses of each APIC struct we're looking to find
        let mut processor_local_apic_virt_address: Option<VirtualAddress> = None;
        let mut io_apic_virt_address: Option<VirtualAddress> = None;
        let mut interrupt_source_override_virt_address: Option<VirtualAddress> = None;

        // Initial structure header. Found immediately after the the MADTHeader
        let mut apic_structure_header_virt_address =
            VirtualAddress::new(madt_address.inner + (size_of::<MADT>() as u64));

        // ITERATION BOUNDS
        let raw_table_end_address = madt_address.inner + madt.header.length as u64;
        let table_end_virt_address = VirtualAddress::new(raw_table_end_address);
        // ITERATION BOUNDS

        while apic_structure_header_virt_address <= table_end_virt_address {
            let apic_structure_header_ref = apic_structure_header_virt_address.inner as *const APICStructureHeader;
            let apic_structure_header = *apic_structure_header_ref;
            // The header tells us which of the above APIC structs we've read the header of
            match apic_structure_header.entry_type {
                0 => processor_local_apic_virt_address = Some(apic_structure_header_virt_address),
                1 => io_apic_virt_address = Some(apic_structure_header_virt_address),
                2 => interrupt_source_override_virt_address = Some(apic_structure_header_virt_address),
                _ => {} // do nothing, we'll skip the rest for now
            }

            apic_structure_header_virt_address.inner += apic_structure_header.length as u64;
        }

        // the APIC Structures
        let processor_local_apic_opt: Option<ProcessorLocalAPIC>;
        let io_apic_opt: Option<IOAPIC>;
        let interrupt_source_override_opt: Option<InterruptSourceOverride>;

        match processor_local_apic_virt_address {
            Some(virt_addr) => processor_local_apic_opt = Some(*(virt_addr.inner as *const ProcessorLocalAPIC)),
            None => {
                log::warn!("Did not find Processor Local APIC table!");
                processor_local_apic_opt = None;
            }
        }

        match io_apic_virt_address {
            Some(virt_addr) => io_apic_opt = Some(*(virt_addr.inner as *const IOAPIC)),
            None => {
                log::warn!("Did not find IO APIC table!");
                io_apic_opt = None;
            }
        }

        match interrupt_source_override_virt_address {
            Some(virt_addr) => {
                interrupt_source_override_opt = Some(*(virt_addr.inner as *const InterruptSourceOverride))
            }
            None => {
                log::warn!("Did not find InterruptSourceOverride table!");
                interrupt_source_override_opt = None;
            }
        }

        match (processor_local_apic_opt, io_apic_opt, interrupt_source_override_opt) {
            (Some(processor_local_apic), Some(io_apic), Some(interrupt_source_override)) => Some(APICStructures {
                processor_local_apic,
                io_apic,
                interrupt_source_override,
            }),
            _ => None,
        }
    }
}
