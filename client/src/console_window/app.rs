use std::rc::Rc;

use wasm_bindgen::__rt::WasmRefCell;

use crate::rpc::HelloRpc;
use crate::status::VolatileStatus;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ConsoleApp {
    label: String,
    address: String,
    // connection_handle:
    #[serde(skip, default = "default_status")]
    volatile_status: Rc<WasmRefCell<VolatileStatus>>,
    #[serde(skip, default = "default_hello_service")]
    hello_service: Rc<WasmRefCell<HelloRpc>>,
}

fn default_status() -> Rc<WasmRefCell<VolatileStatus>> {
    Rc::new(WasmRefCell::new(VolatileStatus::default()))
}

fn default_hello_service() -> Rc<WasmRefCell<HelloRpc>> {
    Rc::new(WasmRefCell::new(HelloRpc::default()))
}

impl Default for ConsoleApp {
    fn default() -> Self {
        Self {
            label: "DataFusion Console - PWA".to_string(),
            address: "".to_string(),
            volatile_status: default_status(),
            hello_service: default_hello_service(),
        }
    }
}

impl ConsoleApp {
    pub fn get_hello_service(&self) -> wasm_bindgen::__rt::Ref<HelloRpc> {
        self.hello_service.borrow()
    }

    pub fn clone_hello_service_rc(&self) -> Rc<WasmRefCell<HelloRpc>> {
        self.hello_service.clone()
    }
}

impl ConsoleApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn clone_clean_app(&self) -> Self {
        Self {
            label: self.label.clone(),
            address: self.address.clone(),
            ..Default::default()
        }
    }

    pub fn get_status(&self) -> wasm_bindgen::__rt::Ref<VolatileStatus> {
        self.volatile_status.borrow()
    }

    pub fn clone_status_rc(&self) -> Rc<WasmRefCell<VolatileStatus>> {
        self.volatile_status.clone()
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    pub fn get_addr(&self) -> String {
        self.address.clone()
    }

    pub fn modify_addr(&mut self, addr: String) {
        self.address = addr;
    }

    pub fn get_addr_mut(&mut self) -> &mut String {
        &mut self.address
    }
}
