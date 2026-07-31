export interface MetricsSnapshot {
  node_id: string;
  cpu_percent: number;
  ram_percent: number;
  ram_used_bytes: number;
  ram_total_bytes: number;
  disk_percent: number;
  disk_used_bytes: number;
  disk_total_bytes: number;
  net_rx_bytes: number;
  net_tx_bytes: number;
  process_count: number;
  load_avg_1min: number;
  load_avg_5min: number;
  load_avg_15min: number;
  collected_at: string;
}

export interface MetricsHistory {
  data: MetricsSnapshot[];
}

export interface MonitoringOverview {
  total_nodes: number;
  online_nodes: number;
  total_containers: number;
  total_vms: number;
  active_alerts: number;
  avg_cpu_percent: number;
  avg_ram_percent: number;
}

export interface Alert {
  id: string;
  title: string;
  message: string;
  severity: 'critical' | 'warning' | 'info';
  source: string;
  acknowledged: boolean;
  created_at: string;
  acknowledged_at?: string;
}
