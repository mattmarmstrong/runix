//use crate::acpi::fadt::FADT;
use crate::acpi::madt::MADT;
use crate::acpi::xsdp::XSDP;
use crate::acpi::xsdt::XSDT;

pub mod fadt;
pub mod madt;
pub mod sdt;
pub mod xsdp;
pub mod xsdt;

pub fn parse_system_descriptor_tables(raw_xsdp_physical_address: u64) {}
