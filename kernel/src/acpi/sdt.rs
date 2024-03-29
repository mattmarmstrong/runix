use core::ptr::{
    addr_of,
    read_unaligned,
};
use core::slice::from_raw_parts;

use crate::mmu::address::VirtualAddress;

#[repr(transparent)]
pub struct SDTSignature {
    inner: [u8; 4],
}

impl SDTSignature {
    pub const FADT: SDTSignature = SDTSignature { inner: *b"FACP" };
    pub const MADT: SDTSignature = SDTSignature { inner: *b"APIC" };
    pub const SSDT: SDTSignature = SDTSignature { inner: *b"SSDT" };
    pub const XSDT: SDTSignature = SDTSignature { inner: *b"XSDT" };
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SDTHeader {
    // Remove these pub declaration. We shouldn't need it outside of this module
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    oemid: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

impl core::fmt::Display for SDTHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let raw_signature = addr_of!(self.signature);
        let raw_length = addr_of!(self.length);
        let raw_revision = addr_of!(self.revision);
        let raw_checksum = addr_of!(self.checksum);
        let raw_oemid = addr_of!(self.oemid);
        let raw_oem_table_id = addr_of!(self.oem_table_id);
        let raw_oem_revision = addr_of!(self.oem_revision);
        let raw_creator_id = addr_of!(self.creator_id);
        let raw_creator_revision = addr_of!(self.creator_revision);
        unsafe {
            let signature = read_unaligned(raw_signature);
            let length = read_unaligned(raw_length);
            let revision = read_unaligned(raw_revision);
            let checksum = read_unaligned(raw_checksum);
            let oemid = read_unaligned(raw_oemid);
            let oem_table_id = read_unaligned(raw_oem_table_id);
            let oem_revision = read_unaligned(raw_oem_revision);
            let creator_id = read_unaligned(raw_creator_id);
            let creator_revision = read_unaligned(raw_creator_revision);
            f.write_fmt(format_args!(
                "SDT Header Values:\nSignature: {}\nLength: {:#X}\nVersion: {}\nChecksum: {:#X}\nOEMID: {}\nOEM Table ID: {}\nOEM Version: {}\nCreator ID: {}\nCreator Version: {}",
                core::str::from_utf8(&signature).unwrap(),
                length,
                revision,
                checksum,
                core::str::from_utf8(&oemid).unwrap(),
                core::str::from_utf8(&oem_table_id).unwrap(),
                oem_revision,
                creator_id,
                creator_revision,
            ))
        }
    }
}

// The SDTHeader
impl SDTHeader {
    pub fn valid_signature(&self, sdt_signature: &SDTSignature) -> bool {
        self.signature == sdt_signature.inner
    }

    pub fn valid_checksum(&self, raw_sdt_start_address: usize) -> bool {
        let virtual_start_address = VirtualAddress::with_kernel_base_offset(raw_sdt_start_address);
        let raw_sdt_byte_slice =
            unsafe { from_raw_parts(virtual_start_address.inner as *const _, self.length as usize) };
        let checksum = raw_sdt_byte_slice
            .iter()
            .fold(0, |sum: u8, byte| sum.wrapping_add(*byte));
        checksum == 0
    }

    // We should read the header directly, then parse it to build the specific SDT struct
    pub unsafe fn try_read_from_phys_addr(
        raw_sdt_physical_address: usize,
        sdt_signature: &SDTSignature,
    ) -> Result<Self, SDTHeaderError> {
        let sdt_virtual_address = VirtualAddress::with_kernel_base_offset(raw_sdt_physical_address);
        let raw_sdt_header = sdt_virtual_address.inner as *const Self;
        let sdt_header = *raw_sdt_header;
        // TODO: compute and validate the checksum as well
        match (
            sdt_header.valid_signature(sdt_signature),
            sdt_header.valid_checksum(raw_sdt_physical_address),
        ) {
            (true, true) => Ok(sdt_header),
            (false, true) => Err(SDTHeaderError::SDTSignatureValidationError),
            (true, false) => Err(SDTHeaderError::SDTChecksumValidationError),
            (false, false) => Err(SDTHeaderError::SDTHeaderNotFoundError),
        }
    }
}

#[derive(Debug)]
pub enum SDTHeaderError {
    SDTHeaderNotFoundError,
    SDTSignatureValidationError,
    SDTChecksumValidationError,
}

impl core::fmt::Display for SDTHeaderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SDTHeaderNotFoundError => f.write_str("SDT Validation Error: SDT Not Found"),
            Self::SDTChecksumValidationError => f.write_str("SDT Validation Error: Invalid Checksum"),
            Self::SDTSignatureValidationError => f.write_str("SDT Validation Error: Invalid Signature"),
        }
    }
}

pub trait SystemDescriptorTable {
    unsafe fn read_from_raw_address(raw_sdt_physical_address: usize) -> Self;
}
