use core::arch::asm;

pub unsafe fn flush(address: u64) {
    asm!("invlpg [{}]", in(reg) address, options(nostack, preserves_flags));
}

pub unsafe fn flush_all() {
    let raw_cr3_value: u64;
    asm!("mov {}, cr3 ", out(reg) raw_cr3_value, options(nostack, preserves_flags));
    asm!("mov cr3, {}", in(reg) raw_cr3_value, options(nostack, preserves_flags));
}
