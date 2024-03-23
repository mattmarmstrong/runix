use conquer_once::spin::OnceCell;

use crate::acpi::madt::MADT;
use crate::acpi::sdt::SDTSignature;
use crate::acpi::sdt::SystemDescriptorTable;
use crate::acpi::xsdp::XSDP;
use crate::acpi::xsdt::XSDT;

pub mod fadt;
pub mod madt;
pub mod sdt;
pub mod xsdp;
pub mod xsdt;

pub static ACPI_TABLES: OnceCell<ACPITables> = OnceCell::uninit();

#[derive(Debug)]
#[repr(C)]
pub struct ACPITables {
    pub xsdt: XSDT,
    pub madt: MADT,
    // fadt: FADT
}

impl ACPITables {
    // Should read this straight from the boot info (UEFI/BIOS)
    unsafe fn read_acpi_tables(raw_xsdp_physical_address: usize) -> ACPITables {
        let xsdp = XSDP::init(raw_xsdp_physical_address);
        let xsdt = XSDT::read_from_raw_address(xsdp.xsdt_address);
        let raw_madt_physical_address = xsdt.try_get_raw_sdt_table_address(&SDTSignature::MADT).unwrap();
        let madt = MADT::read_from_raw_address(raw_madt_physical_address);
        ACPITables { xsdt, madt }
    }
}

pub fn read_acpi_tables(raw_xsdp_physical_address: usize) {
    let acpi_tables = unsafe { ACPITables::read_acpi_tables(raw_xsdp_physical_address) };
    ACPI_TABLES.get_or_init(move || acpi_tables);
    log::info!("Successfully parsed ACPI tables");
}
