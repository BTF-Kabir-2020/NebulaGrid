pub struct DockerManager;

impl DockerManager {
    pub async fn connect() -> Result<Self, String> {
        Err("Docker daemon not available".to_string())
    }
}
