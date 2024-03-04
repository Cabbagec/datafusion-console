use eframe;
use log::{debug, error, info};
use wasm_bindgen_futures;
use web_sys;

use crate::console_window::set_current_url;

pub mod client;
mod console_window;
mod custom_widgets;
mod rpc;
pub mod status;
mod template;

fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    info!("starting...");

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
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
    let w = web_sys::window().expect("failed to find window");
    set_current_url(w.location().host().expect("failed to get host"));
}
