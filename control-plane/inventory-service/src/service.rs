use std::collections::HashMap;
use std::sync::Mutex;

use chrono::Utc;
use uuid::Uuid;

use crate::models::*;

pub struct AppState {
    pub items: Vec<InventoryItem>,
}

impl AppState {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

pub struct InventoryService {
    state: Mutex<AppState>,
}

impl InventoryService {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(AppState::new()),
        }
    }

    pub fn register_item(&self, req: RegisterItemRequest) -> InventoryItem {
        let now = Utc::now();
        let item = InventoryItem {
            id: Uuid::new_v4(),
            asset_tag: req
                .asset_tag
                .unwrap_or_else(|| format!("AST-{}", &Uuid::new_v4().to_string()[..8])),
            name: req.name,
            item_type: req.item_type,
            manufacturer: req.manufacturer,
            model: req.model,
            serial_number: req.serial_number,
            location: req.location,
            rack_id: req.rack_id,
            rack_position: req.rack_position,
            status: "active".into(),
            metadata: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        };
        self.state.lock().unwrap().items.push(item.clone());
        item
    }

    pub fn list_items(&self, query: ListInventoryQuery) -> PaginatedResponse<InventoryItem> {
        let state = self.state.lock().unwrap();
        let mut filtered: Vec<InventoryItem> = state.items.clone();

        if let Some(ref t) = query.item_type {
            filtered.retain(|i| i.item_type == *t);
        }
        if let Some(ref s) = query.status {
            filtered.retain(|i| i.status == *s);
        }
        if let Some(ref loc) = query.location {
            filtered.retain(|i| i.location.as_deref() == Some(loc.as_str()));
        }
        if let Some(ref search) = query.search {
            let s = search.to_lowercase();
            filtered.retain(|i| {
                i.name.to_lowercase().contains(&s)
                    || i.asset_tag.to_lowercase().contains(&s)
                    || i.serial_number
                        .as_ref()
                        .map(|x| x.to_lowercase().contains(&s))
                        .unwrap_or(false)
            });
        }

        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
        paginate(filtered, page, per_page)
    }

    pub fn get_item(&self, id: Uuid) -> Option<InventoryItem> {
        self.state
            .lock()
            .unwrap()
            .items
            .iter()
            .find(|i| i.id == id)
            .cloned()
    }

    pub fn delete_item(&self, id: Uuid) -> bool {
        let mut state = self.state.lock().unwrap();
        let before = state.items.len();
        state.items.retain(|i| i.id != id);
        state.items.len() < before
    }

    pub fn summary(&self) -> InventorySummary {
        let state = self.state.lock().unwrap();
        let mut by_type: HashMap<String, i64> = HashMap::new();
        let mut by_status: HashMap<String, i64> = HashMap::new();
        let mut by_location: HashMap<String, i64> = HashMap::new();

        for item in &state.items {
            *by_type.entry(item.item_type.clone()).or_default() += 1;
            *by_status.entry(item.status.clone()).or_default() += 1;
            let loc = item.location.clone().unwrap_or_else(|| "unknown".into());
            *by_location.entry(loc).or_default() += 1;
        }

        InventorySummary {
            total_items: state.items.len() as i64,
            by_type: serde_json::to_value(by_type).unwrap_or_default(),
            by_status: serde_json::to_value(by_status).unwrap_or_default(),
            by_location: serde_json::to_value(by_location).unwrap_or_default(),
        }
    }
}

fn paginate<T: serde::Serialize>(
    items: Vec<T>,
    page: i64,
    per_page: i64,
) -> PaginatedResponse<T> {
    let total = items.len() as i64;
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i64;
    let offset = ((page - 1) * per_page) as usize;
    let data = items.into_iter().skip(offset).take(per_page as usize).collect();
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
