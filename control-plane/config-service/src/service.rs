use std::sync::Mutex;

use chrono::Utc;
use uuid::Uuid;

use crate::models::*;

pub struct AppState {
    pub entries: Vec<ConfigEntry>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

pub struct ConfigService {
    state: Mutex<AppState>,
}

impl ConfigService {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(AppState::new()),
        }
    }

    pub fn set_config(&self, req: SetConfigRequest) -> ConfigEntry {
        let mut state = self.state.lock().unwrap();
        let group = req.group.unwrap_or_else(|| "default".into());
        let now = Utc::now();

        if let Some(existing) = state.entries.iter_mut().find(|e| e.key == req.key) {
            existing.value = req.value;
            existing.group = group;
            if let Some(desc) = req.description {
                existing.description = Some(desc);
            }
            if let Some(enc) = req.encrypted {
                existing.encrypted = enc;
            }
            existing.version += 1;
            existing.updated_at = now;
            return existing.clone();
        }

        let entry = ConfigEntry {
            id: Uuid::new_v4(),
            key: req.key,
            value: req.value,
            group,
            description: req.description,
            version: 1,
            encrypted: req.encrypted.unwrap_or(false),
            created_at: now,
            updated_at: now,
        };
        state.entries.push(entry.clone());
        entry
    }

    pub fn get_config(&self, key: &str) -> Option<ConfigEntry> {
        self.state
            .lock()
            .unwrap()
            .entries
            .iter()
            .find(|e| e.key == key)
            .cloned()
    }

    pub fn list_config(&self, query: ListConfigQuery) -> PaginatedResponse<ConfigEntry> {
        let state = self.state.lock().unwrap();
        let mut filtered: Vec<ConfigEntry> = state.entries.clone();

        if let Some(ref group) = query.group {
            filtered.retain(|e| e.group == *group);
        }
        if let Some(ref search) = query.search {
            let s = search.to_lowercase();
            filtered.retain(|e| {
                e.key.to_lowercase().contains(&s)
                    || e.description
                        .as_ref()
                        .map(|d| d.to_lowercase().contains(&s))
                        .unwrap_or(false)
            });
        }

        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
        paginate(filtered, page, per_page)
    }

    pub fn list_groups(&self) -> Vec<ConfigGroup> {
        let state = self.state.lock().unwrap();
        let mut map: std::collections::HashMap<String, (i64, chrono::DateTime<Utc>)> =
            std::collections::HashMap::new();
        for entry in &state.entries {
            let slot = map
                .entry(entry.group.clone())
                .or_insert((0, entry.created_at));
            slot.0 += 1;
            if entry.created_at < slot.1 {
                slot.1 = entry.created_at;
            }
        }
        map.into_iter()
            .map(|(name, (count, created_at))| ConfigGroup {
                name,
                description: None,
                entry_count: count,
                created_at,
            })
            .collect()
    }

    pub fn delete_config(&self, key: &str) -> bool {
        let mut state = self.state.lock().unwrap();
        let before = state.entries.len();
        state.entries.retain(|e| e.key != key);
        state.entries.len() < before
    }
}

fn paginate<T: serde::Serialize>(items: Vec<T>, page: i64, per_page: i64) -> PaginatedResponse<T> {
    let total = items.len() as i64;
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i64;
    let offset = ((page - 1) * per_page) as usize;
    let data = items
        .into_iter()
        .skip(offset)
        .take(per_page as usize)
        .collect();
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
