use tokio::sync::Notify;
use wasm_bindgen::__rt::WasmRefCell;

pub struct VolatileStatus {
    pub close_notify: Notify,
    pub connected: bool,
    pub mode: WasmRefCell<Mode>,
    // server
    pub pause_server_yields: WasmRefCell<bool>,
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
            mode: WasmRefCell::new(Mode::Console),
            pause_server_yields: WasmRefCell::new(false),
        }
    }
}

impl VolatileStatus {
    // pub(crate) fn borrow_mode(&mut self) -> &mut Mode {
    //     &mut self.mode
    // }
}
