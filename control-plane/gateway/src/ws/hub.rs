use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use serde_json::json;

#[derive(Clone)]
pub struct WsHub {
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<String>>>>,
}

impl WsHub {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn sender_for(&self, topic: &str) -> broadcast::Sender<String> {
        let mut channels = self.channels.write().await;
        channels
            .entry(topic.to_string())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(256);
                tx
            })
            .clone()
    }

    pub async fn subscribe(&self, topic: &str) -> broadcast::Receiver<String> {
        self.sender_for(topic).await.subscribe()
    }

    pub async fn publish(&self, topic: &str, message: &str) {
        let tx = self.sender_for(topic).await;
        let _ = tx.send(message.to_string());
    }

    pub async fn publish_typed(&self, topic: &str, event_type: &str, data: &serde_json::Value) {
        let payload = json!({ "type": event_type, "data": data }).to_string();
        self.publish(topic, &payload).await;
    }
}
