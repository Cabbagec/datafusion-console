use std::sync::RwLock;

use eframe::{Frame, Storage, wasm_bindgen};
use egui::{Color32, Context, RichText, text::LayoutJob, TextEdit, TextFormat, Widget};
use lazy_static::lazy_static;
use log::{error, info};
use serde;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::client::Client;

pub static default_server_addr: &'static str = "localhost:8080";

lazy_static! {
    static ref CURRENT_URL: RwLock<String> = RwLock::new("".to_string());
}

#[wasm_bindgen]
pub fn set_current_url(url: String) {
    if let Ok(mut current_url) = CURRENT_URL.try_write() {
        *current_url = url;
    } else {
        error!("failed to set current url as {url}");
    }
    info!("current location: {}", *CURRENT_URL.read().unwrap());
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ConsoleApp {
    label: String,
    address: String,
    connected: bool,
    // connection_handle:
}

impl Default for ConsoleApp {
    fn default() -> Self {
        Self {
            label: "DataFusion Console - PWA".to_string(),
            address: "".to_string(),
            connected: false,
        }
    }
}

impl ConsoleApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for ConsoleApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                //
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    egui::widgets::global_dark_light_mode_switch(ui);
                    ui.separator();
                    ui.label(RichText::new("Server").strong());

                    let mut layout = LayoutJob::default();
                    layout.append(
                        "ws://",
                        0.0,
                        TextFormat {
                            color: if ui.visuals().dark_mode {
                                Color32::LIGHT_BLUE
                            } else {
                                Color32::DARK_BLUE
                            },
                            ..Default::default()
                        },
                    );
                    ui.label(layout);

                    TextEdit::singleline(&mut self.address)
                        .char_limit(256)
                        .desired_width(150.0)
                        .text_color(if self.connected {
                            Color32::LIGHT_GREEN
                        } else {
                            Color32::LIGHT_RED
                        })
                        .hint_text(if let Ok(r) = CURRENT_URL.try_read() {
                            r.to_string()
                        } else {
                            default_server_addr.to_string()
                        })
                        .ui(ui);

                    let connect_btn_text = if self.connected {
                        "Disconnect"
                    } else {
                        "Connect"
                    };

                    if ui.button(connect_btn_text).highlight().clicked() {
                        // connect
                        let addr = if !self.address.is_empty() {
                            self.address.clone()
                        } else {
                            if let Ok(r) = CURRENT_URL.try_read() {
                                self.address = r.to_string();
                                self.address.clone()
                            } else {
                                self.address = default_server_addr.to_string();
                                self.address.clone()
                            }
                        };
                        let server_addr = addr.clone();

                        // todo: connect/disconnect here
                        self.connected = !self.connected;
                        if self.connected {
                            spawn_local(async move {
                                let client = Client::new(format!("ws://{server_addr}"));
                                let _ = client.connect().await;
                            });
                            info!("try to connect to {addr}");
                        } else {
                            info!("try to disconnect from {addr}");
                        }
                    }
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.menu_button("File", |ui| {
                ui.button("Help");
                ui.button("Close");
            })
        });
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        eframe::set_value(
            _storage,
            &self.label,
            &Self {
                label: self.label.clone(),
                address: self.address.clone(),
                ..Default::default()
            },
        );
    }

    // fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
    //     todo!()
    // }
    //
    // fn auto_save_interval(&self) -> Duration {
    //     todo!()
    // }
    //
    // fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
    //     todo!()
    // }
    //
    // fn persist_egui_memory(&self) -> bool {
    //     todo!()
    // }
}
