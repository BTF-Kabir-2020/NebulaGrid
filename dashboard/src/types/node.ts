export interface Node {
  id: string;
  hostname: string;
  ip_address: string;
  status: 'online' | 'offline' | 'warning' | 'error';
  cpu_percent: number;
  ram_percent: number;
  disk_percent: number;
  os_name: string;
  cpu_cores?: number;
  ram_total_bytes?: number;
  disk_total_bytes?: number;
  last_seen_at: string;
}

export interface NodeMetrics {
  node_id: string;
  cpu_percent: number;
  ram_percent: number;
  disk_percent: number;
  net_rx_bytes: number;
  net_tx_bytes: number;
  collected_at: string;
}
