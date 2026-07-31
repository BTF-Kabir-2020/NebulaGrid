export interface VirtualMachine {
  id: string;
  name: string;
  status: 'running' | 'stopped' | 'paused';
  os_type: string;
  cpu_cores: number;
  ram_mb: number;
  disk_gb: number;
  node_id: string;
  ip_address: string | null;
}

export interface Snapshot {
  name: string;
  created_at: string;
  size_bytes: number;
}
