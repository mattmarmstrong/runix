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

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct ProcessorLocalAPIC {
    apic_struct_header: APICStructureHeader,
    pub processor_id: u8,
    pub lapic_id: u8,
    flags: u32,
}

impl ProcessorLocalAPIC {
    fn cpu_active_flag(&self) -> bool {
        self.flags == 1
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IOAPIC {
    apic_struct_header: APICStructureHeader,
    pub io_apic_id: u8,
    _reserved: u8,
    pub io_apic_physical_address: u32,
    pub global_system_interrupt_base: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct InterruptSourceOverride {
    apic_struct_header: APICStructureHeader,
    bus: u8,
    source: u8,
    pub global_system_interrupt: u32,
    pub mps_inti_flags: u16,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct LocalAPICAddressOverride {
    // If this exists in the MADT, use this apic address
    apic_struct_header: APICStructureHeader,
    _reserved: u16,
    pub local_apic_address_64: u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MADTHeader {
    pub lapic_address: u32,
    pub multiple_apic_flags: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct APICHeaders {
    // This doesn't represent the entirety of the MADT. These are the headers that we can parse
    // to read the data about the CPUs APIC Structures that exist adjacent in memory
    pub sdt_header: SDTHeader,
    pub madt_header: MADTHeader,
}

impl APICHeaders {
    unsafe fn read_from_raw_address(raw_madt_physical_address: u64) -> Self {
        let sdt_header = SDTHeader::try_read_from_phys_addr(raw_madt_physical_address, &SDTSignature::MADT).unwrap();
        let size_of_sdt_header = size_of::<SDTHeader>() as u64;
        let madt_header_phys_addr = PhysicalAddress::new(raw_madt_physical_address + size_of_sdt_header);
        let madt_header_virt_addr = phys_to_virt_address(madt_header_phys_addr);
        let madt_header_ref = madt_header_virt_addr.inner as *const MADTHeader;
        let madt_header = *madt_header_ref;
        APICHeaders {
            sdt_header,
            madt_header,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct APICStructures {
    pub processor_local_apic_records: [Option<ProcessorLocalAPIC>; 16],
    pub io_apic_records: [Option<IOAPIC>; 2],
    pub interrupt_source_override_records: [Option<InterruptSourceOverride>; 8],
    pub local_apic_address_override: Option<LocalAPICAddressOverride>,
}

impl APICStructures {
    pub unsafe fn read_apic_structures(raw_madt_physical_address: u64) -> APICStructures {
        let apic_headers = APICHeaders::read_from_raw_address(raw_madt_physical_address);
        let madt_virtual_address = phys_to_virt_address(PhysicalAddress::new(raw_madt_physical_address));

        // Initial structure header. Found immediately after the the MADTHeader
        let mut apic_structure_header_address =
            VirtualAddress::new(madt_virtual_address.inner + (size_of::<APICHeaders>() as u64));

        // Struct initalization. I choose these values for no real reason.
        // TODO: NEED TO ALLOCATE HEAP SPACE FOR THESE AND USE A VEC<>
        // AFTER MM!
        let mut processor_local_apic_records: [Option<ProcessorLocalAPIC>; 16] = [None; 16];
        let mut lapic_records_index: usize = 0;

        let mut io_apic_records: [Option<IOAPIC>; 2] = [None; 2];
        let mut io_apic_records_index: usize = 0;

        let mut interrupt_source_override_records: [Option<InterruptSourceOverride>; 8] = [None; 8];
        let mut iso_records_index: usize = 0;

        let mut local_apic_address_override: Option<LocalAPICAddressOverride> = None;

        // ITERATION BOUNDS
        let raw_table_end_address = madt_virtual_address.inner + apic_headers.sdt_header.length as u64;
        let table_end_virt_address = VirtualAddress::new(raw_table_end_address);
        // ITERATION BOUNDS

        while apic_structure_header_address.inner <= table_end_virt_address.inner {
            let apic_structure_header_ref = apic_structure_header_address.inner as *const APICStructureHeader;
            let apic_structure_header = *apic_structure_header_ref;
            // The header tells us which of the above APIC structs we've read the header of
            match apic_structure_header.entry_type {
                0 => {
                    let processor_lapic_record = *(apic_structure_header_address.inner as *const ProcessorLocalAPIC);
                    if processor_lapic_record.cpu_active_flag() {
                        processor_local_apic_records[lapic_records_index] = Some(processor_lapic_record);
                        lapic_records_index += 1;
                    }
                }
                1 => {
                    let io_apic_record = *(apic_structure_header_address.inner as *const IOAPIC);
                    io_apic_records[io_apic_records_index] = Some(io_apic_record);
                    io_apic_records_index += 1;
                }
                2 => {
                    let iso_record = *(apic_structure_header_address.inner as *const InterruptSourceOverride);
                    interrupt_source_override_records[iso_records_index] = Some(iso_record);
                    iso_records_index += 1;
                }
                5 => {
                    local_apic_address_override =
                        Some(*(apic_structure_header_address.inner as *const LocalAPICAddressOverride));
                }
                _ => {} // do nothing, we'll skip the rest for now
            }

            apic_structure_header_address.inner += apic_structure_header.length as u64;
        }

        APICStructures {
            processor_local_apic_records,
            io_apic_records,
            interrupt_source_override_records,
            local_apic_address_override,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MADT {
    pub apic_headers: APICHeaders,
    pub apic_structures: APICStructures,
}

impl SystemDescriptorTable for MADT {
    unsafe fn read_from_raw_address(raw_madt_physical_address: u64) -> Self {
        let apic_headers = APICHeaders::read_from_raw_address(raw_madt_physical_address);
        let apic_structures = APICStructures::read_apic_structures(raw_madt_physical_address);
        MADT {
            apic_headers,
            apic_structures,
        }
    }
}
