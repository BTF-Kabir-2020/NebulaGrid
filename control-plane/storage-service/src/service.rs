use std::sync::Mutex;

use chrono::Utc;
use uuid::Uuid;

use crate::models::*;

pub struct AppState {
    pub pools: Vec<StoragePool>,
    pub volumes: Vec<Volume>,
    pub snapshots: Vec<Snapshot>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            pools: Vec::new(),
            volumes: Vec::new(),
            snapshots: Vec::new(),
        }
    }
}

pub struct StorageService {
    state: Mutex<AppState>,
}

impl StorageService {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(AppState::new()),
        }
    }

    pub fn create_pool(&self, req: CreatePoolRequest) -> StoragePool {
        let now = Utc::now();
        let pool = StoragePool {
            id: Uuid::new_v4(),
            name: req.name,
            pool_type: req.pool_type,
            total_bytes: req.total_bytes,
            used_bytes: 0,
            free_bytes: req.total_bytes,
            mount_path: req.mount_path,
            status: "online".into(),
            labels: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        };
        self.state.lock().unwrap().pools.push(pool.clone());
        pool
    }

    pub fn list_pools(&self, page: i64, per_page: i64) -> PaginatedResponse<StoragePool> {
        let state = self.state.lock().unwrap();
        paginate(&state.pools, page, per_page)
    }

    pub fn create_volume(&self, req: CreateVolumeRequest) -> Result<Volume, String> {
        let mut state = self.state.lock().unwrap();
        let pool = state
            .pools
            .iter_mut()
            .find(|p| p.id == req.pool_id)
            .ok_or_else(|| "pool not found".to_string())?;
        if pool.free_bytes < req.size_bytes {
            return Err("insufficient free space".into());
        }
        pool.used_bytes += req.size_bytes;
        pool.free_bytes -= req.size_bytes;
        pool.updated_at = Utc::now();
        let now = Utc::now();
        let volume = Volume {
            id: Uuid::new_v4(),
            pool_id: req.pool_id,
            name: req.name,
            size_bytes: req.size_bytes,
            volume_type: req.volume_type.unwrap_or_else(|| "block".into()),
            format: req.format.unwrap_or_else(|| "qcow2".into()),
            mount_path: None,
            attached_to: None,
            status: "available".into(),
            created_at: now,
            updated_at: now,
        };
        state.volumes.push(volume.clone());
        Ok(volume)
    }

    pub fn list_volumes(&self, page: i64, per_page: i64) -> PaginatedResponse<Volume> {
        let state = self.state.lock().unwrap();
        paginate(&state.volumes, page, per_page)
    }

    pub fn create_snapshot(&self, volume_id: Uuid, name: String) -> Result<Snapshot, String> {
        let mut state = self.state.lock().unwrap();
        let volume = state
            .volumes
            .iter()
            .find(|v| v.id == volume_id)
            .ok_or_else(|| "volume not found".to_string())?;
        let snap = Snapshot {
            id: Uuid::new_v4(),
            volume_id,
            name,
            size_bytes: volume.size_bytes,
            snapshot_type: "manual".into(),
            created_at: Utc::now(),
        };
        state.snapshots.push(snap.clone());
        Ok(snap)
    }
}

fn paginate<T: Clone + serde::Serialize>(
    items: &[T],
    page: i64,
    per_page: i64,
) -> PaginatedResponse<T> {
    let page = page.max(1);
    let per_page = per_page.clamp(1, 100);
    let total = items.len() as i64;
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i64;
    let offset = ((page - 1) * per_page) as usize;
    let data = items.iter().skip(offset).take(per_page as usize).cloned().collect();
    PaginatedResponse {
        data,
        page,
        per_page,
        total,
        total_pages,
        has_next: page < total_pages,
        has_prev: page > 1,
    }
}
