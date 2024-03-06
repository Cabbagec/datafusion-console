use tokio::sync::Notify;

use crate::RefCell;

pub struct VolatileStatus {
    pub close_notify: Notify,
    pub connected: bool,
    pub mode: RefCell<Mode>,
    // server
    pub pause_server_yields: RefCell<bool>,
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
            pause_server_yields: RefCell::new(false),
        }
    }
}

impl VolatileStatus {
    // pub(crate) fn borrow_mode(&mut self) -> &mut Mode {
    //     &mut self.mode
    // }
}
