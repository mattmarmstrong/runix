pub mod asm;
pub mod gdt;
pub mod tss;

use lazy_static::lazy_static;

use crate::segmentation::asm::*;
use crate::segmentation::gdt::{
    GlobalDescriptorTable,
    SegmentDescriptor,
    Selectors,
};
use crate::segmentation::tss::*;

pub const DPL_0: u8 = 0x00; // kernel-space privilege ring
pub const DPL_3: u8 = 0x03; // user-space privilege ring

#[derive(Debug)]
#[repr(C)]
pub struct GDTWithSegmentSelectors {
    table: GlobalDescriptorTable,
    pub selectors: Selectors,
}

lazy_static! {
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        unsafe { tss.init_interrupt_stack_table(DOUBLE_FAULT_STACK_TABLE_INDEX, DOUBLE_FAULT_STACK) }
        unsafe { tss.init_interrupt_stack_table(PAGE_FAULT_STACK_TABLE_INDEX, PAGE_FAULT_STACK) }
        tss
    };
}

lazy_static! {
    pub static ref GDT: GDTWithSegmentSelectors = {
        let mut gdt = GlobalDescriptorTable::new();
        let null_segment_selector = gdt.get_entry(0);
        let kernel_code_selector = gdt.set_entry(1, SegmentDescriptor::kernel_code_segment_descriptor(), DPL_0);
        let kernel_data_selector = gdt.set_entry(2, SegmentDescriptor::kernel_data_segment_descriptor(), DPL_0);
        let user_code_selector = gdt.set_entry(3, SegmentDescriptor::user_code_segment_descriptor(), DPL_3);
        let user_data_selector = gdt.set_entry(4, SegmentDescriptor::user_data_segment_descriptor(), DPL_3);
        let (tss_system_segment_low, tss_system_segment_high) = SegmentDescriptor::tss_system_segment(&TSS);
        let tss_selector = gdt.set_entry(5, tss_system_segment_low, DPL_0);
        let _ = gdt.set_entry(6, tss_system_segment_high, DPL_0); // Ignore the high segment
        let selectors = Selectors {
            null_segment_selector,
            kernel_code_selector,
            kernel_data_selector,
            user_code_selector,
            user_data_selector,
            tss_selector
        };
        GDTWithSegmentSelectors { table: gdt, selectors }

    };
}

pub fn init_gdt() {
    unsafe {
        GlobalDescriptorTable::load_gdt(&GDT.table.address());
        set_cs(GDT.selectors.kernel_code_selector);
        set_ss(GDT.selectors.kernel_data_selector);
        set_ds(GDT.selectors.kernel_data_selector);
        set_es(GDT.selectors.kernel_data_selector);
        set_fs(GDT.selectors.kernel_data_selector);
        set_gs(GDT.selectors.kernel_data_selector);
        load_task_register(GDT.selectors.tss_selector);
    }
    log::info!("Loaded GDT and segment registers");
}
