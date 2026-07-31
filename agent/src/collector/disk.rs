use sysinfo::Disks;

#[allow(dead_code)]
pub struct DiskInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub percent: f64,
}

#[allow(dead_code)]
pub fn get_disk_info() -> DiskInfo {
    let disks = Disks::new_with_refreshed_list();
    let mut total_bytes: u64 = 0;
    let mut used_bytes: u64 = 0;

    for disk in disks.list() {
        total_bytes += disk.total_space();
        used_bytes += disk.total_space() - disk.available_space();
    }

    let percent = if total_bytes > 0 {
        (used_bytes as f64 / total_bytes as f64) * 100.0
    } else {
        0.0
    };

    DiskInfo {
        total_bytes,
        used_bytes,
        percent,
    }
}
