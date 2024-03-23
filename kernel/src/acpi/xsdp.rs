use core::ptr::{
    addr_of,
    read_unaligned,
};
use core::str;

use crate::mmu::address::VirtualAddress;

pub const XSDP_SIGNATURE: [u8; 8] = *b"RSD PTR ";

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct XSDP {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    rsdt_address: u32,
    length: u32,
    pub xsdt_address: usize,
    extended_checksum: u8,
    _reserved: [u8; 3],
}

#[derive(Debug)]
pub enum XSDPError {
    XSDReadError,
}

impl core::fmt::Display for XSDPError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Need a match statement here if we add any enum variants above
        f.write_str("Failed to read RSD/XSD from ptr!")
    }
}

impl core::fmt::Display for XSDP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // We have to jump through a ton of hoops because these fields are unaligned
        // https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html
        let raw_signature = addr_of!(self.signature);
        let raw_checksum = addr_of!(self.checksum);
        let raw_oemid = addr_of!(self.oemid);
        let raw_revision = addr_of!(self.revision);
        let raw_rsdt_address = addr_of!(self.rsdt_address);
        let raw_length = addr_of!(self.length);
        let raw_xsdt_address = addr_of!(self.xsdt_address);
        let raw_extended_checksum = addr_of!(self.extended_checksum);
        unsafe {
            let signature = read_unaligned(raw_signature);
            let checksum = read_unaligned(raw_checksum);
            let oemid = read_unaligned(raw_oemid);
            let revision = read_unaligned(raw_revision);
            let rsdt_address = read_unaligned(raw_rsdt_address);
            let length = read_unaligned(raw_length);
            let xsdt_address = read_unaligned(raw_xsdt_address);
            let extended_checksum = read_unaligned(raw_extended_checksum);

            f.write_fmt(format_args!(
                "XSDP Values:\nSignature: {}\nChecksum: {}\nOEMID: {}\nACPI Version: {}\nRDST Address: {:#X}\nLength: {}\nXSDT Address: {:#X}\nExtended Checksum: {}",
                str::from_utf8(&signature).unwrap(),
                checksum,
                str::from_utf8(&oemid).unwrap(),
                revision,
                rsdt_address,
                length,
                xsdt_address,
                extended_checksum
            ))
        }
    }
}

impl XSDP {
    fn valid_signature(&self) -> bool {
        self.signature == XSDP_SIGNATURE
    }

    #[allow(dead_code)]
    fn valid_extended_checksum(&self) -> bool {
        // TODO: FIX-ME
        (self.extended_checksum & 0xFF) == 0
    }

    unsafe fn try_read_from_raw_address(raw_xsdp_physical_address: usize) -> Result<XSDP, XSDPError> {
        let rsdp_virtual_address = VirtualAddress::with_kernel_base_offset(raw_xsdp_physical_address);
        let xsdp_ref = rsdp_virtual_address.inner as *const XSDP;
        let xsdp: XSDP = *xsdp_ref;
        match xsdp.valid_signature() {
            true => Ok(xsdp),
            false => Err(XSDPError::XSDReadError),
        }
    }

    pub fn init(raw_rsdp_physical_address: usize) -> XSDP {
        unsafe { XSDP::try_read_from_raw_address(raw_rsdp_physical_address).unwrap() }
    }
}
