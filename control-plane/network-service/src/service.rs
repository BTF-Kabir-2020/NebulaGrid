use std::sync::Mutex;

use chrono::Utc;
use uuid::Uuid;

use crate::models::*;

pub struct AppState {
    pub networks: Vec<Network>,
    pub firewall_rules: Vec<FirewallRule>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            networks: Vec::new(),
            firewall_rules: Vec::new(),
        }
    }
}

pub struct NetworkService {
    state: Mutex<AppState>,
}

impl NetworkService {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(AppState::new()),
        }
    }

    pub fn create_network(&self, req: CreateNetworkRequest) -> Network {
        let now = Utc::now();
        let network = Network {
            id: Uuid::new_v4(),
            name: req.name,
            subnet: req.subnet,
            gateway: req.gateway,
            vlan_id: req.vlan_id,
            network_type: req.network_type.unwrap_or_else(|| "bridge".into()),
            status: "active".into(),
            labels: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        };
        self.state.lock().unwrap().networks.push(network.clone());
        network
    }

    pub fn list_networks(&self, page: i64, per_page: i64) -> PaginatedResponse<Network> {
        let state = self.state.lock().unwrap();
        paginate(&state.networks, page, per_page)
    }

    pub fn get_network(&self, id: Uuid) -> Option<Network> {
        self.state
            .lock()
            .unwrap()
            .networks
            .iter()
            .find(|n| n.id == id)
            .cloned()
    }

    pub fn delete_network(&self, id: Uuid) -> bool {
        let mut state = self.state.lock().unwrap();
        let before = state.networks.len();
        state.networks.retain(|n| n.id != id);
        state.firewall_rules.retain(|r| r.network_id != id);
        state.networks.len() < before
    }

    pub fn add_firewall_rule(
        &self,
        req: CreateFirewallRuleRequest,
    ) -> Result<FirewallRule, String> {
        let mut state = self.state.lock().unwrap();
        if !state.networks.iter().any(|n| n.id == req.network_id) {
            return Err("network not found".into());
        }
        let rule = FirewallRule {
            id: Uuid::new_v4(),
            network_id: req.network_id,
            name: req.name,
            direction: req.direction,
            protocol: req.protocol,
            source: req.source,
            destination: req.destination,
            ports: req.ports,
            action: req.action,
            priority: req.priority.unwrap_or(100),
            enabled: true,
            created_at: Utc::now(),
        };
        state.firewall_rules.push(rule.clone());
        Ok(rule)
    }

    pub fn attach(&self, req: NetworkAttachment) -> Result<NetworkAttachment, String> {
        let state = self.state.lock().unwrap();
        if !state.networks.iter().any(|n| n.id == req.network_id) {
            return Err("network not found".into());
        }
        Ok(req)
    }

    pub fn list_firewall_rules(&self, network_id: Uuid) -> Vec<FirewallRule> {
        self.state
            .lock()
            .unwrap()
            .firewall_rules
            .iter()
            .filter(|r| r.network_id == network_id)
            .cloned()
            .collect()
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
