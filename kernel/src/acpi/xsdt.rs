use core::mem::size_of;
use core::slice::from_raw_parts;

use crate::acpi::sdt::{
    SDTHeader,
    SDTSignature,
    SystemDescriptorTable,
};
use crate::mmu::virtual_address::VirtualAddress;

#[derive(Debug)]
#[repr(C, packed)]
pub struct XSDT {
    header: SDTHeader,
    sdt_address_table: &'static [u64],
}

impl SystemDescriptorTable for XSDT {
    unsafe fn read_from_raw_address(raw_xsdt_physical_address: u64) -> Self {
        let header = SDTHeader::try_read_from_phys_addr(raw_xsdt_physical_address, &SDTSignature::XSDT).unwrap();
        let size_of_sdt_header = size_of::<SDTHeader>() as u64;
        let sdt_address_table_size = header.length as u64 - size_of_sdt_header;
        let sdt_address_table_length = (sdt_address_table_size / 8) as usize;
        let sdt_address_table_virt_addr =
            VirtualAddress::with_kernel_base_offset(raw_xsdt_physical_address + size_of_sdt_header);
        let sdt_address_table = from_raw_parts(sdt_address_table_virt_addr.inner as *const _, sdt_address_table_length);
        XSDT {
            header,
            sdt_address_table,
        }
    }
}

impl XSDT {
    pub unsafe fn try_get_raw_sdt_table_address(&self, sdt_signature: &SDTSignature) -> Option<u64> {
        for raw_sdt_address in self.sdt_address_table {
            if let Ok(_) = SDTHeader::try_read_from_phys_addr(*raw_sdt_address, sdt_signature) {
                return Some(*raw_sdt_address);
            } else {
                continue;
            }
        }
        None
    }
}
