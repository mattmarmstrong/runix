use lazy_static::lazy_static;

use crate::cpu::cpu_info::CPUInfo;

pub mod cpu_info;
pub mod ioapic;
pub mod lapic;

lazy_static! {
    pub static ref CPU_INFO: CPUInfo = unsafe { CPUInfo::parse_raw_cpuid() };
}
