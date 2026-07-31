use std::collections::HashMap;
use std::sync::Mutex;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::*;

pub struct AppState {
    pub nodes: Vec<Node>,
    pub metrics: HashMap<Uuid, Vec<NodeMetrics>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            metrics: HashMap::new(),
        }
    }
}

pub struct NodeService {
    state: Mutex<AppState>,
}

impl NodeService {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(AppState::new()),
        }
    }

    pub fn register_node(&self, req: RegisterNodeRequest) -> Node {
        let now = Utc::now();
        let node = Node {
            id: Uuid::new_v4(),
            hostname: req.hostname,
            ip_address: req.ip_address,
            status: "online".to_string(),
            os_name: req.os_name,
            os_version: req.os_version,
            cpu_cores: req.cpu_cores,
            ram_total_bytes: req.ram_total_bytes,
            disk_total_bytes: req.disk_total_bytes,
            labels: serde_json::json!({}),
            last_seen_at: Some(now),
            created_at: now,
            updated_at: now,
        };

        let mut state = self.state.lock().unwrap();
        state.nodes.push(node.clone());
        node
    }

    pub fn list_nodes(&self, query: ListNodesQuery) -> PaginatedResponse<Node> {
        let state = self.state.lock().unwrap();

        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(20).max(1).min(100);

        let mut filtered: Vec<Node> = state.nodes.clone();

        if let Some(ref status) = query.status {
            filtered.retain(|n| n.status == *status);
        }

        if let Some(ref search) = query.search {
            let s = search.to_lowercase();
            filtered.retain(|n| {
                n.hostname.to_lowercase().contains(&s)
                    || n.ip_address.contains(&s)
            });
        }

        let total = filtered.len() as i64;
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        let offset = ((page - 1) * per_page) as usize;
        let data: Vec<Node> = filtered.into_iter().skip(offset).take(per_page as usize).collect();

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

    pub fn get_node(&self, id: Uuid) -> Option<Node> {
        let state = self.state.lock().unwrap();
        state.nodes.iter().find(|n| n.id == id).cloned()
    }

    pub fn update_last_seen(&self, id: Uuid) {
        let mut state = self.state.lock().unwrap();
        if let Some(node) = state.nodes.iter_mut().find(|n| n.id == id) {
            node.last_seen_at = Some(Utc::now());
            node.updated_at = Utc::now();
        }
    }

    pub fn store_metrics(&self, node_id: Uuid, mut metrics: NodeMetrics) {
        metrics.node_id = node_id;
        let mut state = self.state.lock().unwrap();

        if let Some(node) = state.nodes.iter_mut().find(|n| n.id == node_id) {
            node.last_seen_at = Some(Utc::now());
            node.updated_at = Utc::now();
        }

        state
            .metrics
            .entry(node_id)
            .or_default()
            .push(metrics);
    }

    pub fn get_latest_metrics(&self, node_id: Uuid) -> Option<NodeMetrics> {
        let state = self.state.lock().unwrap();
        state
            .metrics
            .get(&node_id)
            .and_then(|v| v.last().cloned())
    }

    pub fn get_metrics_history(
        &self,
        node_id: Uuid,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Vec<NodeMetrics> {
        let state = self.state.lock().unwrap();
        match state.metrics.get(&node_id) {
            Some(records) => records
                .iter()
                .filter(|m| {
                    let after_from = from.map(|f| m.collected_at >= f).unwrap_or(true);
                    let before_to = to.map(|t| m.collected_at <= t).unwrap_or(true);
                    after_from && before_to
                })
                .cloned()
                .collect(),
            None => Vec::new(),
        }
    }
}
