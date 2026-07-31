use sysinfo::System;

#[allow(dead_code)]
pub fn get_cpu_usage(sys: &System) -> f64 {
    sys.global_cpu_usage() as f64
}
