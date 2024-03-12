use egui::{Color32, RichText, TextEdit, Widget};
use log::info;

use crate::spawn_local;

use super::{Client, ConsoleApp, get_current_host, Mode};

impl ConsoleApp {
    pub fn draw_top_menu_in_ui(self: &mut ConsoleApp, ui: &mut egui::Ui) {
        self.draw_top_menu_in_ctx(ui.ctx());
    }

    pub fn draw_top_menu_in_ctx(self: &mut ConsoleApp, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_menu")
            .default_height(36.0)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        let (addr_txt_color, connect_btn_text) = if self.get_status().connected {
                            (Color32::LIGHT_GREEN, "Disconnect")
                        } else {
                            (Color32::LIGHT_RED, "Connect")
                        };
                        // let scheme_color = if ui.visuals().dark_mode {
                        //     Color32::LIGHT_BLUE
                        // } else {
                        //     Color32::DARK_BLUE
                        // };
                        let current_host = get_current_host();

                        egui::widgets::global_dark_light_mode_switch(ui);
                        ui.separator();
                        ui.label(RichText::new("Client mode:").strong());
                        {
                            let s_ref = self.get_status();
                            let mode_mut = &mut *(s_ref.mode.borrow_mut());
                            egui::ComboBox::from_id_source("mode")
                                .width(24.0)
                                .selected_text(mode_mut.to_string())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(mode_mut, Mode::Console, "Console");
                                    ui.selectable_value(mode_mut, Mode::Monitor, "Monitor");
                                });
                        }

                        ui.separator();
                        ui.label("Address: ");
                        TextEdit::singleline(self.get_addr_mut())
                            .char_limit(256)
                            .desired_width(150.0)
                            .text_color(addr_txt_color)
                            .hint_text(get_current_host())
                            .ui(ui);

                        if ui.button(connect_btn_text).highlight().clicked() {
                            // connect
                            let current_addr = self.get_addr();
                            if current_addr.is_empty() {
                                self.modify_addr(current_host);
                            }

                            let addr = self.get_addr();
                            if !self.get_status().connected {
                                let mut client =
                                    Client::new(format!("ws://{addr}"), self.clone_status_rc());
                                client.add_service(self.clone_hello_service_rc());
                                spawn_local(async move {
                                    let _ = client.connect().await;
                                });
                                info!("try to connect to {addr}");
                            } else {
                                self.get_status().close_notify.notify_one();
                                info!("try to disconnect from {}", addr);
                            }
                        };

                        // ui.separator();
                        // ui.label("Select Mode: ");
                        // let s_ref = self.get_status();
                        // let mode_mut = &mut *(s_ref.mode.borrow_mut());
                        // ui.add_enabled_ui(s_ref.connected, |ui| {
                        //     ui.selectable_value(mode_mut, Mode::Console, "Console")
                        //         .highlight();
                        //     ui.selectable_value(mode_mut, Mode::Monitor, "Monitor")
                        //         .highlight();
                        // });
                    });
                });
            });
    }
}
