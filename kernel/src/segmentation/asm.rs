use core::arch::asm;

use crate::segmentation::gdt::SegmentSelector;

// Mission Statement: "Stay in Rust-space, at all costs"

pub unsafe fn set_cs(selector: SegmentSelector) {
    let selector_as_u64: u64 = selector.inner as u64;
    asm!(
    "push {0}", // Push the selector value
    "lea {1}, [1f + rip]", // Load the effective address, stick it in any old register
    "push {1}", // push that address
    "retfq", // return from these instructions
    "1:", // label definition
    in(reg) selector_as_u64,
    lateout(reg) _,
    options(preserves_flags),
    );
}

pub unsafe fn get_cs() -> SegmentSelector {
    let segment_value: u16;
    unsafe {
        asm!("mov {0:x}, cs", out(reg) segment_value, options(nomem, nostack, preserves_flags));
    }
    SegmentSelector { inner: segment_value }
}

pub unsafe fn set_ss(selector: SegmentSelector) {
    asm!(
    "mov ss, {0:x}", // {0:x} -> I don't know what this does. It supresses a compiler warning though.
    in(reg) selector.inner,
    options(nostack, preserves_flags)
    );
}

pub unsafe fn set_ds(selector: SegmentSelector) {
    asm!(
    "mov ds, {0:x}",
    in(reg) selector.inner,
    options(nostack, preserves_flags)
    );
}

pub unsafe fn set_es(selector: SegmentSelector) {
    asm!(
    "mov es, {0:x}",
    in(reg) selector.inner,
    options(nostack, preserves_flags)
    );
}

pub unsafe fn set_fs(selector: SegmentSelector) {
    asm!(
    "mov fs, {0:x}",
    in(reg) selector.inner,
    options(nostack, preserves_flags)
    );
}

pub unsafe fn set_gs(selector: SegmentSelector) {
    asm!(
    "mov gs, {0:x}",
    in(reg) selector.inner,
    options(nostack, preserves_flags)
    );
}

pub unsafe fn load_task_register(selector: SegmentSelector) {
    asm!(
    "ltr {0:x}",
    in(reg) selector.inner,
    options(nostack, preserves_flags)
    );
}
