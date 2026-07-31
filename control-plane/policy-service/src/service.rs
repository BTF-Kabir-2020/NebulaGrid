use std::sync::Mutex;

use chrono::Utc;
use uuid::Uuid;

use crate::models::*;

pub struct AppState {
    pub policies: Vec<Policy>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }
}

pub struct PolicyService {
    state: Mutex<AppState>,
}

impl PolicyService {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(AppState::new()),
        }
    }

    pub fn create_policy(&self, req: CreatePolicyRequest) -> Policy {
        let now = Utc::now();
        let mut rules = req.rules;
        for rule in &mut rules {
            if rule.id.is_none() {
                rule.id = Some(Uuid::new_v4());
            }
        }
        let policy = Policy {
            id: Uuid::new_v4(),
            name: req.name,
            description: req.description,
            policy_type: req.policy_type,
            scope: req.scope,
            target_selector: serde_json::json!({}),
            rules,
            enforcement: req.enforcement.unwrap_or_else(|| "enforce".into()),
            priority: req.priority.unwrap_or(100),
            enabled: true,
            created_at: now,
            updated_at: now,
        };
        self.state.lock().unwrap().policies.push(policy.clone());
        policy
    }

    pub fn list_policies(&self, page: i64, per_page: i64) -> PaginatedResponse<Policy> {
        let state = self.state.lock().unwrap();
        paginate(&state.policies, page, per_page)
    }

    pub fn get_policy(&self, id: Uuid) -> Option<Policy> {
        self.state
            .lock()
            .unwrap()
            .policies
            .iter()
            .find(|p| p.id == id)
            .cloned()
    }

    pub fn delete_policy(&self, id: Uuid) -> bool {
        let mut state = self.state.lock().unwrap();
        let before = state.policies.len();
        state.policies.retain(|p| p.id != id);
        state.policies.len() < before
    }

    pub fn evaluate(&self, req: EvaluateRequest) -> Vec<PolicyEvaluationResult> {
        let state = self.state.lock().unwrap();
        let mut results = Vec::new();

        let mut policies: Vec<_> = state
            .policies
            .iter()
            .filter(|p| p.enabled)
            .cloned()
            .collect();
        policies.sort_by_key(|p| p.priority);

        for policy in policies {
            let mut matched = Vec::new();
            let mut deny = false;

            for rule in &policy.rules {
                let ctx_val = req.context.get(&rule.field);
                let matches = match rule.operator.as_str() {
                    "eq" => ctx_val == Some(&rule.value),
                    "neq" => ctx_val != Some(&rule.value),
                    "exists" => ctx_val.is_some(),
                    _ => false,
                };
                if matches {
                    matched.push(format!("{} {} {:?}", rule.field, rule.operator, rule.value));
                    if rule.effect == "deny" {
                        deny = true;
                    }
                }
            }

            if matched.is_empty() {
                continue;
            }

            let allowed = !deny;
            results.push(PolicyEvaluationResult {
                policy_id: policy.id,
                policy_name: policy.name.clone(),
                action: req.action.clone(),
                allowed,
                reason: Some(if allowed {
                    format!("allowed by policy for {}", req.resource_type)
                } else {
                    format!("denied by policy for {}", req.resource_id)
                }),
                matched_rules: matched,
            });

            if !allowed && policy.enforcement == "enforce" {
                break;
            }
        }

        if results.is_empty() {
            results.push(PolicyEvaluationResult {
                policy_id: Uuid::nil(),
                policy_name: "default-allow".into(),
                action: req.action,
                allowed: true,
                reason: Some("no matching policy".into()),
                matched_rules: vec![],
            });
        }

        results
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
