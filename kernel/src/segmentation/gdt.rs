use core::arch::asm;
use core::mem::size_of;

use crate::segmentation::tss::TaskStateSegment;
use crate::vmm::VirtualAddress;

#[derive(Debug, Clone, Copy)]
pub struct GlobalDescriptorTable {
    descriptor_table: [SegmentDescriptor; 7],
}

#[repr(C, packed(2))]
pub struct GdtPointer {
    limit: u16,
    base: VirtualAddress,
}

impl GlobalDescriptorTable {
    pub fn new() -> GlobalDescriptorTable {
        GlobalDescriptorTable {
            descriptor_table: [SegmentDescriptor::null_segment_descriptor(); 7],
        }
    }

    pub fn get_entry(&mut self, index: usize) -> SegmentSelector {
        let descriptor = self.descriptor_table[index];
        let segment_dpl = descriptor.get_requested_privilege_level();
        SegmentSelector::new(index as u16, segment_dpl)
    }

    pub fn set_entry(&mut self, index: usize, segment_desc: SegmentDescriptor, dpl: u8) -> SegmentSelector {
        self.descriptor_table[index] = segment_desc;
        SegmentSelector::new(index as u16, dpl)
    }

    pub fn address(&self) -> GdtPointer {
        let limit = (self.descriptor_table.len() * size_of::<SegmentDescriptor>() - 1) as u16;
        let base = VirtualAddress::new(self.descriptor_table.as_ptr() as u64);
        GdtPointer { limit, base }
    }

    pub unsafe fn load_gdt(gdt_ptr: &GdtPointer) {
        asm!("lgdt [{}]", in(reg) gdt_ptr, options(readonly, nostack, preserves_flags))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl SegmentDescriptor {
    // See the section on Segment Descriptors here -> https://wiki.osdev.org/Global_Descriptor_Table
    pub const fn null_segment_descriptor() -> SegmentDescriptor {
        SegmentDescriptor {
            limit_low: 0x0000,
            base_low: 0x0000,
            base_middle: 0x00,
            access: 0x00,
            granularity: 0x00,
            base_high: 0x00,
        }
    }

    pub const fn kernel_code_segment_descriptor() -> Self {
        SegmentDescriptor {
            limit_low: 0xFFFF,
            base_low: 0x0000,
            base_middle: 0x00,
            access: 0x9A,
            granularity: 0xAF,
            base_high: 0x00,
        }
    }

    pub const fn kernel_data_segment_descriptor() -> Self {
        SegmentDescriptor {
            limit_low: 0xFFFF,
            base_low: 0x0000,
            base_middle: 0x00,
            access: 0x92,
            granularity: 0xAF,
            base_high: 0x00,
        }
    }

    pub const fn user_code_segment_descriptor() -> Self {
        SegmentDescriptor {
            limit_low: 0xFFFF,
            base_low: 0x0000,
            base_middle: 0x00,
            access: 0xFA,
            granularity: 0xAF,
            base_high: 0x00,
        }
    }

    pub const fn user_data_segment_descriptor() -> Self {
        SegmentDescriptor {
            limit_low: 0xFFFF,
            base_low: 0x0000,
            base_middle: 0x00,
            access: 0xF2,
            granularity: 0xAF,
            base_high: 0x00,
        }
    }

    pub fn tss_system_segment(tss: &'static TaskStateSegment) -> (Self, Self) {
        let tss_ptr = tss.address();
        let tss_system_segment_low = SegmentDescriptor {
            limit_low: (size_of::<TaskStateSegment>() - 1) as u16,
            base_low: (tss_ptr.inner & 0x0000_FFFF) as u16,
            base_middle: ((tss_ptr.inner & 0x00FF_0000) >> 16) as u8,
            access: 0xE9 as u8,
            granularity: 0x00 as u8,
            base_high: ((tss_ptr.inner & 0xFF00_0000) >> 24) as u8,
        };
        let tss_system_segment_high = SegmentDescriptor {
            limit_low: ((tss_ptr.inner & 0x0000_FFFF_0000_0000) >> 32) as u16,
            base_low: ((tss_ptr.inner & 0xFFFF_0000_0000_0000) >> 48) as u16,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        };
        (tss_system_segment_low, tss_system_segment_high)
    }

    // convenience method
    pub fn get_requested_privilege_level(&self) -> u8 {
        let rpl_bit_mask: u8 = 0b0110_0000;
        self.access & rpl_bit_mask
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct SegmentSelector {
    pub inner: u16,
}

impl SegmentSelector {
    pub fn new(index: u16, dpl: u8) -> SegmentSelector {
        SegmentSelector {
            inner: index << 3 | dpl as u16,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Selectors {
    pub null_segment_selector: SegmentSelector,
    pub kernel_code_selector: SegmentSelector,
    pub kernel_data_selector: SegmentSelector,
    pub user_code_selector: SegmentSelector,
    pub user_data_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}
