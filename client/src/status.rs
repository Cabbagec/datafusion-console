use tokio::sync::Notify;

use crate::RefCell;

pub struct VolatileStatus {
    pub close_notify: Notify,
    pub connected: bool,
    pub edit_ctx_name: RefCell<String>,
    pub mode: RefCell<Mode>,
    // server
    pub pause_server_yields: RefCell<bool>,
    pub context_names: RefCell<Vec<String>>,
    pub current_context_name: RefCell<Option<String>>,
}

#[derive(PartialEq)]
pub enum Mode {
    Monitor,
    Console,
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
            context_names: RefCell::new(vec![]),
            current_context_name: RefCell::new(None),
        }
    }
}

impl VolatileStatus {
    // pub fn mut_ctx_name(&mut self) -> &mut String {
    //     &mut self.edit_ctx_name
    // }
}
