use crate::acpi::sdt::{
    SDTHeader,
    SystemDescriptorTable,
};

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct XSDT {
    header: SDTHeader,
}

impl SystemDescriptorTable for XSDT {}
