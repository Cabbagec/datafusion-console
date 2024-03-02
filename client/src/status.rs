use tokio::sync::Notify;

pub struct VolatileStatus {
    pub close_notify: Notify,
    pub connected: bool,
}

impl Default for VolatileStatus {
    fn default() -> Self {
        Self {
            close_notify: Notify::new(),
            connected: false,
        }
    }
}
