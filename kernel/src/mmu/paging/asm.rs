use core::arch::asm;

#[inline]
pub unsafe fn get_raw_pml4_ptr() -> u64 {
    let raw_pml4_address: u64;
    asm!("mov {}, cr3", out(reg) raw_pml4_address, options(nomem, nostack, preserves_flags));
    raw_pml4_address
}

#[inline]
pub unsafe fn flush(address: u64) {
    asm!("invlpg [{}]", in(reg) address, options(nostack, preserves_flags));
}

pub unsafe fn flush_all() {
    let raw_cr3_value: u64;
    asm!("mov {}, cr3 ", out(reg) raw_cr3_value, options(nostack, preserves_flags));
    asm!("mov cr3, {}", in(reg) raw_cr3_value, options(nostack, preserves_flags));
}
