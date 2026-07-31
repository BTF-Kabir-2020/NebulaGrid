pub struct PendingCommand {
    pub id: String,
    pub command: String,
    pub args: Vec<String>,
}

pub struct CommandWatcher;

impl CommandWatcher {
    pub async fn watch() -> Result<Option<PendingCommand>, String> {
        Ok(None)
    }

    pub async fn ack_command(_id: &str) -> Result<(), String> {
        Ok(())
    }
}
