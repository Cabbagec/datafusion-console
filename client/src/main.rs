pub use std::cell::{Ref, RefCell};

use eframe;
use log::info;
// pub use wasm_bindgen::__rt::{Ref, WasmRefCell as RefCell};
pub use wasm_bindgen_futures::spawn_local;

use crate::console_window::set_current_host;

pub mod client;
mod console_window;
mod custom_widgets;
mod rpc;
mod status;
mod template;

fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    info!("starting...");

    let web_options = eframe::WebOptions::default();
    spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|c| {
                    // Box::new(template::TemplateApp::new(c))
                    Box::new(console_window::ConsoleApp::new(c))
                }),
            )
            .await
            .expect("failed to start template");
        info!("runner");
    });
    info!("spawning done!");
    spawn_local(set_current_host(
        web_sys::window()
            .expect("failed to find window")
            .location()
            .host()
            .expect("failed to get host"),
    ));
}
