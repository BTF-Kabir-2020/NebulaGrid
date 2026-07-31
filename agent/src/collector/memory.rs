use sysinfo::System;

#[allow(dead_code)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub percent: f64,
}

#[allow(dead_code)]
pub fn get_memory_info(sys: &System) -> MemoryInfo {
    let total = sys.total_memory();
    let used = sys.used_memory();
    let percent = if total > 0 {
        (used as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    MemoryInfo {
        total_bytes: total,
        used_bytes: used,
        percent,
    }
}
