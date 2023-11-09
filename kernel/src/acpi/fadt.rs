use crate::acpi::sdt::{
    SDTHeader,
    SystemDescriptorTable,
};

// TODO: Everything FADT related
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct FADT {
    header: SDTHeader,
}
