use core::arch::asm;

// MSR values: https://sandpile.org/x86/msr.html
pub const IA32_APIC_MSR_BASE: u32 = 0x1B;

pub unsafe fn read_msr_value(msr_base: u32) -> usize {
    let (high_bytes, low_bytes): (u32, u32);
    asm!("rdmsr", out("edx") high_bytes, out("eax") low_bytes, in("ecx") msr_base, options(nomem));
    ((high_bytes as usize) << 32) | (low_bytes as usize)
}
