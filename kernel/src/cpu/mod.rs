use lazy_static::lazy_static;
use raw_cpuid::CpuId;

#[derive(Debug)]
pub enum CpuVendor {
    Intel,
    Amd,
}

impl CpuVendor {
    fn to_str(&self) -> &str {
        match self {
            CpuVendor::Intel => "Intel",
            CpuVendor::Amd => "AMD",
        }
    }
}

// Wrapper struct for chip info from raw_cpuid::CpuId
#[derive(Debug)]
pub struct CpuInfo {
    cpu_vendor: Option<CpuVendor>,
    acpi_enabled: bool,
    sse3_enabled: bool,
    apic_enabled: bool,
    x2apic_enabled: bool,
    apic_id: Option<u8>,
}

impl CpuInfo {
    pub fn new() -> CpuInfo {
        CpuInfo {
            cpu_vendor: None,
            acpi_enabled: false,
            sse3_enabled: false,
            apic_enabled: false,
            x2apic_enabled: false,
            apic_id: None,
        }
    }

    pub unsafe fn parse_raw_cpuid() -> CpuInfo {
        let raw_cpuid = CpuId::new();
        let cpu_vendor = raw_cpuid.get_vendor_info().unwrap();
        let cpu_features = raw_cpuid.get_feature_info().unwrap();
        let mut cpu_info = CpuInfo::new();
        match cpu_vendor.as_str() {
            "GenuineIntel" => cpu_info.cpu_vendor = Some(CpuVendor::Intel),
            "AuthenticAMD" => cpu_info.cpu_vendor = Some(CpuVendor::Amd),
            _ => unreachable!(),
        }
        cpu_info.acpi_enabled = cpu_features.has_acpi();
        cpu_info.sse3_enabled = cpu_features.has_sse3();
        cpu_info.apic_enabled = cpu_features.has_apic();
        cpu_info.x2apic_enabled = cpu_features.has_x2apic();
        if cpu_info.apic_enabled {
            cpu_info.apic_id = Some(cpu_features.initial_local_apic_id());
        }
        cpu_info
    }
}

lazy_static! {
    pub static ref CPU_INFO: CpuInfo = unsafe { CpuInfo::parse_raw_cpuid() };
}

impl core::fmt::Display for CPU_INFO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "CPU INFO:\nVendor: {}\nFeature Enabled: ACPI Thermal Control MSRs - {}\nFeature Enabled: SSE3 - {}\nFeature Enabled: APIC - {}\nFeature Enabled: X2APIC - {}\nInitial APIC ID: {}",
            self.cpu_vendor.as_ref().unwrap().to_str(),
            self.acpi_enabled,
            self.sse3_enabled,
            self.apic_enabled,
            self.x2apic_enabled,
            self.apic_id.unwrap(),
        ))
    }
}

pub fn log_cpu_info() {
    log::info!("{}", CPU_INFO);
}
