use sysinfo::System;

#[allow(dead_code)]
pub struct ProcessInfo {
    pub count: u32,
}

#[allow(dead_code)]
pub fn get_process_count(sys: &System) -> u32 {
    sys.processes().len() as u32
}
