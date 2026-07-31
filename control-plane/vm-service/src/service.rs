use std::sync::Mutex;

use chrono::Utc;
use uuid::Uuid;

use crate::models::*;

pub struct AppState {
    pub vms: Vec<Vm>,
    pub snapshots: Vec<VmSnapshot>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            vms: Vec::new(),
            snapshots: Vec::new(),
        }
    }
}

pub struct VmService {
    state: Mutex<AppState>,
    #[allow(dead_code)]
    proxmox_url: String,
    #[allow(dead_code)]
    proxmox_token: Option<String>,
}

impl VmService {
    pub fn new(proxmox_url: String, proxmox_token: Option<String>) -> Self {
        Self {
            state: Mutex::new(AppState::new()),
            proxmox_url,
            proxmox_token,
        }
    }

    pub fn list_vms(&self, status_filter: Option<String>) -> Vec<Vm> {
        let state = self.state.lock().unwrap();
        match status_filter {
            Some(filter) => state
                .vms
                .iter()
                .filter(|v| v.status == filter)
                .cloned()
                .collect(),
            None => state.vms.clone(),
        }
    }

    pub fn get_vm(&self, id: Uuid) -> Option<Vm> {
        let state = self.state.lock().unwrap();
        state.vms.iter().find(|v| v.id == id).cloned()
    }

    pub fn create_vm(&self, req: CreateVmRequest) -> Vm {
        let now = Utc::now();
        let vm = Vm {
            id: Uuid::new_v4(),
            name: req.name,
            os_type: req.os_type,
            cpu_cores: req.cpu_cores,
            ram_mb: req.ram_mb,
            disk_gb: req.disk_gb,
            status: "stopped".to_string(),
            node_id: req.node_id,
            proxmox_vmid: None,
            created_at: now,
            updated_at: now,
        };

        let mut state = self.state.lock().unwrap();
        state.vms.push(vm.clone());
        vm
    }

    pub fn start_vm(&self, id: Uuid) -> ActionResult {
        let mut state = self.state.lock().unwrap();
        match state.vms.iter_mut().find(|v| v.id == id) {
            Some(vm) => {
                if vm.status == "running" {
                    return ActionResult {
                        success: false,
                        message: format!("VM {} is already running", vm.name),
                    };
                }
                vm.status = "running".to_string();
                vm.updated_at = Utc::now();
                ActionResult {
                    success: true,
                    message: format!("VM {} started", vm.name),
                }
            }
            None => ActionResult {
                success: false,
                message: format!("VM {id} not found"),
            },
        }
    }

    pub fn stop_vm(&self, id: Uuid) -> ActionResult {
        let mut state = self.state.lock().unwrap();
        match state.vms.iter_mut().find(|v| v.id == id) {
            Some(vm) => {
                if vm.status == "stopped" {
                    return ActionResult {
                        success: false,
                        message: format!("VM {} is already stopped", vm.name),
                    };
                }
                vm.status = "stopped".to_string();
                vm.updated_at = Utc::now();
                ActionResult {
                    success: true,
                    message: format!("VM {} stopped", vm.name),
                }
            }
            None => ActionResult {
                success: false,
                message: format!("VM {id} not found"),
            },
        }
    }

    pub fn restart_vm(&self, id: Uuid) -> ActionResult {
        let mut state = self.state.lock().unwrap();
        match state.vms.iter_mut().find(|v| v.id == id) {
            Some(vm) => {
                vm.status = "running".to_string();
                vm.updated_at = Utc::now();
                ActionResult {
                    success: true,
                    message: format!("VM {} restarted", vm.name),
                }
            }
            None => ActionResult {
                success: false,
                message: format!("VM {id} not found"),
            },
        }
    }

    pub fn create_snapshot(&self, vm_id: Uuid, name: String) -> Option<VmSnapshot> {
        let mut state = self.state.lock().unwrap();
        if state.vms.iter().any(|v| v.id == vm_id) {
            let snapshot = VmSnapshot {
                id: Uuid::new_v4(),
                vm_id,
                name,
                created_at: Utc::now(),
            };
            state.snapshots.push(snapshot.clone());
            Some(snapshot)
        } else {
            None
        }
    }

    pub fn list_snapshots(&self, vm_id: Uuid) -> Vec<VmSnapshot> {
        let state = self.state.lock().unwrap();
        state
            .snapshots
            .iter()
            .filter(|s| s.vm_id == vm_id)
            .cloned()
            .collect()
    }
}
