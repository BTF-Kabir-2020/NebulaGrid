pub mod cpu;
pub mod memory;
pub mod disk;
pub mod os;
pub mod network;
pub mod process;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MetricsSnapshot {
    pub cpu_percent: f64,
    pub ram_percent: f64,
    pub ram_used_bytes: u64,
    pub ram_total_bytes: u64,
    pub disk_percent: f64,
    pub disk_used_bytes: u64,
    pub disk_total_bytes: u64,
    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
    pub process_count: u32,
    pub load_avg_1min: f64,
    pub load_avg_5min: f64,
    pub load_avg_15min: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_snapshot_defaults() {
        let s = MetricsSnapshot {
            cpu_percent: 0.0,
            ram_percent: 0.0,
            ram_used_bytes: 0,
            ram_total_bytes: 0,
            disk_percent: 0.0,
            disk_used_bytes: 0,
            disk_total_bytes: 0,
            net_rx_bytes: 0,
            net_tx_bytes: 0,
            process_count: 0,
            load_avg_1min: 0.0,
            load_avg_5min: 0.0,
            load_avg_15min: 0.0,
        };
        assert_eq!(s.cpu_percent, 0.0);
        assert_eq!(s.process_count, 0);
    }
}
