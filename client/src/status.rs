use std::fmt::Display;

use egui::ahash::{HashSet, HashSetExt};
use tokio::sync::Notify;

use crate::RefCell;

pub struct VolatileStatus {
    pub close_notify: Notify,
    pub connected: bool,
    pub edit_ctx_name: RefCell<String>,
    pub mode: RefCell<Mode>,
    // server
    pub pause_server_yields: RefCell<bool>,
    pub context_names: RefCell<HashSet<String>>,
    pub current_context_name: RefCell<Option<String>>,
}

#[derive(PartialEq, Clone)]
pub enum Mode {
    Monitor,
    Console,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Mode::Monitor => "Monitor".to_string(),
            Mode::Console => "Console".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Default for VolatileStatus {
    fn default() -> Self {
        Self {
            close_notify: Notify::new(),
            connected: false,
            mode: RefCell::new(Mode::Console),
            edit_ctx_name: RefCell::new("".to_string()),
            // server
            pause_server_yields: RefCell::new(false),
            context_names: RefCell::new(HashSet::new()),
            current_context_name: RefCell::new(None),
        }
    }
}

impl VolatileStatus {
    // pub fn mut_ctx_name(&mut self) -> &mut String {
    //     &mut self.edit_ctx_name
    // }
}
