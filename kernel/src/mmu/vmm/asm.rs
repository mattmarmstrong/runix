use core::arch::asm;

#[inline]
pub unsafe fn get_raw_pml4_ptr() -> usize {
    let raw_pml4_address: usize;
    asm!("mov {}, cr3", out(reg) raw_pml4_address, options(nomem, nostack, preserves_flags));
    raw_pml4_address
}

#[inline]
pub unsafe fn flush(address: usize) {
    asm!("invlpg [{}]", in(reg) address, options(nostack, preserves_flags));
}

pub unsafe fn flush_all() {
    let raw_cr3_value: usize;
    asm!("mov {}, cr3 ", out(reg) raw_cr3_value, options(nostack, preserves_flags));
    asm!("mov cr3, {}", in(reg) raw_cr3_value, options(nostack, preserves_flags));
}
