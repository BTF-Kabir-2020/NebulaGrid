use sysinfo::System;

#[allow(dead_code)]
pub struct OsInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub architecture: String,
    pub cpu_model: String,
    pub cpu_cores: u32,
}

#[allow(dead_code)]
pub fn get_os_info(sys: &System) -> OsInfo {
    let hostname = System::host_name().unwrap_or_else(|| "unknown".into());

    OsInfo {
        hostname,
        os_name: System::name().unwrap_or_else(|| "unknown".into()),
        os_version: System::os_version().unwrap_or_else(|| "unknown".into()),
        kernel_version: System::kernel_version().unwrap_or_else(|| "unknown".into()),
        architecture: std::env::consts::ARCH.to_string(),
        cpu_model: sys.cpus().first().map(|c| c.brand().to_string()).unwrap_or_else(|| "unknown".into()),
        cpu_cores: sys.physical_core_count().unwrap_or(0) as u32,
    }
}
