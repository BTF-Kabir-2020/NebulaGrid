use sysinfo::Networks;

#[allow(dead_code)]
pub struct NetworkInfo {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

#[allow(dead_code)]
pub fn get_network_info() -> NetworkInfo {
    let networks = Networks::new_with_refreshed_list();
    let mut rx_bytes: u64 = 0;
    let mut tx_bytes: u64 = 0;

    for (_name, data) in &networks {
        rx_bytes += data.total_received();
        tx_bytes += data.total_transmitted();
    }

    NetworkInfo { rx_bytes, tx_bytes }
}
