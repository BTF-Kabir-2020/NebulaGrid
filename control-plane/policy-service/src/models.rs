use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Policy {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub policy_type: String,
    pub scope: String,
    pub target_selector: serde_json::Value,
    pub rules: Vec<PolicyRule>,
    pub enforcement: String,
    pub priority: i32,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PolicyRule {
    pub id: Option<Uuid>,
    pub field: String,
    pub operator: String,
    pub value: serde_json::Value,
    pub effect: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PolicyEvaluationResult {
    pub policy_id: Uuid,
    pub policy_name: String,
    pub action: String,
    pub allowed: bool,
    pub reason: Option<String>,
    pub matched_rules: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePolicyRequest {
    pub name: String,
    pub description: Option<String>,
    pub policy_type: String,
    pub scope: String,
    pub rules: Vec<PolicyRule>,
    pub enforcement: Option<String>,
    pub priority: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct EvaluateRequest {
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub context: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
    pub has_next: bool,
    pub has_prev: bool,
}
