use crate::collector::MetricsSnapshot;

pub struct MetricsReporter {
    server_url: String,
    token: String,
    client: reqwest::Client,
}

impl MetricsReporter {
    pub fn new(server_url: String, token: String) -> Self {
        Self {
            server_url,
            token,
            client: reqwest::Client::new(),
        }
    }

    pub async fn send_metrics(&self, snapshot: &MetricsSnapshot, node_id: &str) -> Result<(), String> {
        let url = format!("{}/api/nodes/{}/metrics", self.server_url, node_id);
        let body = serde_json::json!({
            "cpu_percent": snapshot.cpu_percent,
            "ram_percent": snapshot.ram_percent,
            "ram_used_bytes": snapshot.ram_used_bytes,
            "ram_total_bytes": snapshot.ram_total_bytes,
            "disk_percent": snapshot.disk_percent,
            "disk_used_bytes": snapshot.disk_used_bytes,
            "disk_total_bytes": snapshot.disk_total_bytes,
            "net_rx_bytes": snapshot.net_rx_bytes,
            "net_tx_bytes": snapshot.net_tx_bytes,
            "process_count": snapshot.process_count,
            "collected_at": chrono::Utc::now().to_rfc3339(),
        });

        let resp = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("Gateway returned {status}: {text}"));
        }

        Ok(())
    }
}
