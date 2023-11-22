use conquer_once::spin::OnceCell;

use crate::cpu::cpu_info::CPUInfo;
use crate::cpu::lapic::LocalAPIC;

pub mod cpu_info;
pub mod ioapic;
pub mod lapic;
pub mod msr;
pub mod pit;

pub static LOCAL_APIC: OnceCell<LocalAPIC> = OnceCell::uninit();
pub static CPU_INFO: OnceCell<CPUInfo> = OnceCell::uninit();

pub fn init_cpu_intrinsics() {
    CPU_INFO.get_or_init(move || unsafe { CPUInfo::parse_raw_cpuid() });
    LOCAL_APIC.get_or_init(move || LocalAPIC::initialize_core_lapic());
}
