use std::sync::Mutex;

use chrono::Utc;
use uuid::Uuid;

use crate::models::*;

pub struct MonitoringService {
    alerts: Mutex<Vec<Alert>>,
    prometheus_url: String,
    client: reqwest::Client,
}

impl MonitoringService {
    pub fn new(prometheus_url: String) -> Self {
        let alerts = Mutex::new(vec![
            Alert {
                id: Uuid::new_v4(),
                node_id: Some(Uuid::new_v4()),
                alert_type: "high_cpu".to_string(),
                severity: "critical".to_string(),
                message: "Node CPU usage above 90% threshold".to_string(),
                acknowledged: false,
                created_at: Utc::now(),
                acknowledged_at: None,
            },
            Alert {
                id: Uuid::new_v4(),
                node_id: Some(Uuid::new_v4()),
                alert_type: "disk_full".to_string(),
                severity: "critical".to_string(),
                message: "Node disk usage at 95% capacity".to_string(),
                acknowledged: false,
                created_at: Utc::now(),
                acknowledged_at: None,
            },
            Alert {
                id: Uuid::new_v4(),
                node_id: Some(Uuid::new_v4()),
                alert_type: "high_memory".to_string(),
                severity: "warning".to_string(),
                message: "Node memory usage above 80% threshold".to_string(),
                acknowledged: false,
                created_at: Utc::now(),
                acknowledged_at: None,
            },
        ]);

        let client = reqwest::Client::new();

        Self {
            alerts,
            prometheus_url,
            client,
        }
    }

    pub fn get_overview(&self) -> MonitoringOverview {
        let active = self
            .alerts
            .lock()
            .unwrap()
            .iter()
            .filter(|a| !a.acknowledged)
            .count() as i64;

        MonitoringOverview {
            total_nodes: 12,
            online_nodes: 11,
            total_containers: 47,
            total_vms: 8,
            active_alerts: active,
            avg_cpu_percent: 52.3,
            avg_ram_percent: 67.8,
        }
    }

    pub fn list_alerts(&self, query: ListAlertsQuery) -> Vec<Alert> {
        let alerts = self.alerts.lock().unwrap();

        alerts
            .iter()
            .filter(|a| {
                if let Some(ref severity) = query.severity {
                    if a.severity != *severity {
                        return false;
                    }
                }
                if let Some(acknowledged) = query.acknowledged {
                    if a.acknowledged != acknowledged {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect()
    }

    pub fn acknowledge_alert(&self, id: Uuid) -> Result<Alert, String> {
        let mut alerts = self.alerts.lock().unwrap();

        let alert = alerts
            .iter_mut()
            .find(|a| a.id == id)
            .ok_or_else(|| "Alert not found".to_string())?;

        alert.acknowledged = true;
        alert.acknowledged_at = Some(Utc::now());

        Ok(alert.clone())
    }

    pub async fn prometheus_query(&self, query_str: &str) -> Result<PrometheusResult, String> {
        let url = format!(
            "{}/api/v1/query?query={}",
            self.prometheus_url.trim_end_matches('/'),
            urlencoding(query_str)
        );

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Prometheus request failed: {e}"))?;

        let data: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse Prometheus response: {e}"))?;

        let status = data["status"]
            .as_str()
            .unwrap_or("error")
            .to_string();

        Ok(PrometheusResult {
            status,
            data: data["data"].clone(),
        })
    }
}

fn urlencoding(s: &str) -> String {
    use std::fmt::Write;
    let mut result = String::with_capacity(s.len() * 3);
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            _ => {
                write!(result, "%{:02X}", byte).unwrap();
            }
        }
    }
    result
}
