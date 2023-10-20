use crate::mmu::{
    phys_to_virt_address,
    PhysicalAddress,
};

pub const XSDP_SIGNATURE: [u8; 8] = *b"RSD PTR ";

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct XSDP {
    signature: [u8; 8],
    checksum: u8,
    omeid: [u8; 6],
    revision: u8,
    rsdt_address: u32,
    length: u32,
    xsdt_address: u64,
    extended_checksum: u8,
    _reserved: [u8; 3],
}

impl core::fmt::Display for XSDP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // We have to jump through a ton of hoops because these fields are unaligned
        // https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html
        let raw_signature = core::ptr::addr_of!(self.signature);
        let raw_checksum = core::ptr::addr_of!(self.checksum);
        let raw_omeid = core::ptr::addr_of!(self.omeid);
        let raw_revision = core::ptr::addr_of!(self.revision);
        let raw_rsdt_address = core::ptr::addr_of!(self.rsdt_address);
        let raw_length = core::ptr::addr_of!(self.length);
        let raw_xsdt_address = core::ptr::addr_of!(self.xsdt_address);
        let raw_extended_checksum = core::ptr::addr_of!(self.extended_checksum);
        unsafe {
            let signature = core::ptr::read_unaligned(raw_signature);
            let checksum = core::ptr::read_unaligned(raw_checksum);
            let omeid = core::ptr::read_unaligned(raw_omeid);
            let revision = core::ptr::read_unaligned(raw_revision);
            let rsdt_address = core::ptr::read_unaligned(raw_rsdt_address);
            let length = core::ptr::read_unaligned(raw_length);
            let xsdt_address = core::ptr::read_unaligned(raw_xsdt_address);
            let extended_checksum = core::ptr::read_unaligned(raw_extended_checksum);

            f.write_fmt(format_args!(
                "XSDP Values:\nSignature: {}\nChecksum: {}\nOMEID: {}\nACPI Version: {}\nRDST Address: {:#X}\nLength: {}\nXSDT Address: {:#X}\nExtended Checksum: {}",
                core::str::from_utf8(&signature).unwrap(),
                checksum,
                core::str::from_utf8(&omeid).unwrap(),
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

    fn valid_extended_checksum(&self) -> bool {
        (self.extended_checksum & 0xFF) == 0
    }

    unsafe fn try_read_from_raw_address(raw_rsdp_physical_address: u64) -> Option<XSDP> {
        let rsdp_physical_address = PhysicalAddress::new(raw_rsdp_physical_address);
        let rsdp_virtual_address = phys_to_virt_address(rsdp_physical_address);
        let raw_xsdp = rsdp_virtual_address.inner as *const XSDP;
        let xsdp: XSDP = *raw_xsdp;
        match xsdp.valid_signature() && xsdp.valid_extended_checksum() {
            true => Some(xsdp),
            false => None,
        }
    }

    pub fn init(raw_rsdp_physical_address: u64) -> XSDP {
        unsafe { XSDP::try_read_from_raw_address(raw_rsdp_physical_address).unwrap() }
    }
}
