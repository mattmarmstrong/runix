use crate::mmu::{
    phys_to_virt_address,
    PhysicalAddress,
};

// TODO: if we end up using a bunch of these system tables, break this out into individual files
// TODO: checksum validation
// Error Handling
pub trait SystemDescriptorTable {
    unsafe fn from_raw_physical_address(raw_sdt_physical_address: u64) -> Self
    where
        Self: core::marker::Sized,
        Self: core::marker::Copy,
    {
        let sdt_physical_address = PhysicalAddress::new(raw_sdt_physical_address);
        let sdt_virtual_address = phys_to_virt_address(sdt_physical_address);
        let raw_sdt = sdt_virtual_address.inner as *const Self;
        *raw_sdt
    }
}
#[repr(transparent)]
pub struct SDTSignature {
    inner: [u8; 4],
}

impl SDTSignature {
    pub const FADT_SIGNATURE: SDTSignature = SDTSignature { inner: *b"FACP" };
    pub const MADT_SIGNATURE: SDTSignature = SDTSignature { inner: *b"APIC" };
    pub const SSDT_SIGNATURE: SDTSignature = SDTSignature { inner: *b"SSDT" };
}
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SDTHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    omeid: [u8; 6],
    omeid_table_id: [u8; 8],
    ome_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

// The SDTHeader
impl SDTHeader {
    fn valid_signature(&self, sdt_signature: SDTSignature) -> bool {
        self.signature == sdt_signature.inner
    }

    fn valid_checksum(&self) -> bool {
        (self.checksum & 0xFF) == 0
    }
}

#[derive(Debug)]
pub enum SDTError {
    SDTSignatureValidationError,
    SDTChecksumValidationError,
}

impl core::fmt::Display for SDTError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SDTChecksumValidationError => f.write_str("SDT Validation Error: Invalid Checksum"),
            Self::SDTSignatureValidationError => f.write_str("SDT Validation Error: Invalid Signature"),
        }
    }
}
