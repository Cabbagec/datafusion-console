use std::rc::Rc;
use std::sync::RwLock;

use eframe::{Frame, Storage, wasm_bindgen};
use egui::{Color32, Context, RichText, Stroke, text::LayoutJob, TextEdit, TextFormat, Widget};
use egui_extras::{Size, StripBuilder};
use lazy_static::lazy_static;
use log::{error, info};
use serde;
use wasm_bindgen::__rt::WasmRefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::client::Client;
use crate::rpc::HelloRpc;
use crate::status::{Mode, VolatileStatus};

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
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for ConsoleApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel")
            .default_height(36.0)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
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
                            .text_color(if self.volatile_status.borrow().connected {
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

                        let connect_btn_text = if self.volatile_status.borrow().connected {
                            "Disconnect"
                        } else {
                            "Connect"
                        };

                        if ui.button(connect_btn_text).highlight().clicked() {
                            // connect
                            if !self.address.is_empty() {
                            } else if let Ok(r) = CURRENT_URL.try_read() {
                                self.address = r.to_string();
                            } else {
                                self.address = default_server_addr.to_string();
                            };

                            // todo: connect/disconnect here
                            if !self.volatile_status.borrow().connected {
                                let status = self.volatile_status.clone();
                                let addr = self.address.clone();
                                let mut client = Client::new(format!("ws://{addr}"), status);
                                client.add_service(self.hello_service.clone());
                                spawn_local(async move {
                                    let _ = client.connect().await;
                                });
                                info!("try to connect to {}", self.address);
                            } else {
                                self.volatile_status.borrow().close_notify.notify_one();
                                info!("try to disconnect from {}", self.address);
                            }
                        };
                        ui.separator();
                        ui.label("Select Mode: ");
                        let s_ref = self.volatile_status.borrow();
                        let mode_mut = &mut *(s_ref.mode.borrow_mut());
                        ui.add_enabled_ui(s_ref.connected, |ui| {
                            ui.selectable_value(mode_mut, Mode::Console, "Console")
                                .highlight();
                            ui.selectable_value(mode_mut, Mode::Monitor, "Monitor")
                                .highlight();
                        });
                    });
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.visuals_mut().clip_rect_margin += 100.0;
            // let b = StripBuilder::new(ui).size()

            ui.add_enabled_ui(self.volatile_status.borrow().connected, |ui| {
                StripBuilder::new(ui)
                    .size(Size::remainder().at_least(720.0))
                    .vertical(|mut strip_v| {
                        strip_v.strip(|cols_builder| {
                            cols_builder
                                .size(Size::relative(0.2).at_least(200.0).at_most(240.0))
                                .size(Size::relative(0.6).at_least(640.0).at_most(640.0))
                                .size(Size::relative(0.2).at_least(200.0).at_most(240.0))
                                .horizontal(|mut col| {
                                    // left col
                                    col.cell(|ui| {
                                        ui.painter().rect_stroke(
                                            ui.available_rect_before_wrap(),
                                            3.0,
                                            Stroke::new(
                                                0.8,
                                                if ui.visuals().dark_mode {
                                                    Color32::LIGHT_GRAY
                                                } else {
                                                    Color32::GRAY
                                                },
                                            ),
                                        );
                                        ui.separator();
                                        ui.vertical_centered(|ui| {
                                            ui.label("DataFusion Runtime Context")
                                        });

                                        // egui::Grid::new("menu_grid")
                                        //     .num_columns(1)
                                        //     .striped(true)
                                        //     .show(ui, |ui| {
                                        //         ui.horizontal_wrapped(|ui| {
                                        //             ui.selectable_value(
                                        //                 mode_mut,
                                        //                 Mode::Console,
                                        //                 "Console",
                                        //             )
                                        //             .highlight();
                                        //             ui.selectable_value(
                                        //                 mode_mut,
                                        //                 Mode::Monitor,
                                        //                 "Monitor",
                                        //             )
                                        //             .highlight();
                                        //         });
                                        //     });
                                        ui.separator();
                                    });
                                    // center col
                                    col.cell(|ui| {
                                        ui.horizontal_top(|ui| {
                                            if ui.button("Hello").clicked() {
                                                self.hello_service.borrow().say_hello();
                                                info!("click!");
                                            }
                                            ui.label("right");
                                        });
                                    });
                                    // right col
                                    col.cell(|ui| {
                                        ui.painter().rect_filled(
                                            ui.available_rect_before_wrap(),
                                            0.0,
                                            Color32::LIGHT_YELLOW,
                                        );
                                        ui.label("right");
                                    });
                                });
                        });
                    });
            });
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