use eframe;
use log::{debug, error, info};
use wasm_bindgen_futures;

pub mod client;
mod console_window;
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
    info!("spawning done!")
}
