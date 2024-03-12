use egui::{Button, Color32, RichText, Ui};
use egui_extras::{Size, StripBuilder};
use log::info;

use crate::status::Mode;

use super::ConsoleApp;

impl ConsoleApp {
    pub fn draw_center_panel_on_mode_in_ui(&mut self, ui: &mut Ui) {
        let current_mode = self.get_status().mode.borrow().clone();
        match current_mode {
            Mode::Console => self.draw_console_in_ui(ui),
            Mode::Monitor => {}
        }
    }

    fn draw_console_in_ui(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            StripBuilder::new(ui)
                .size(Size::relative(0.1))
                .size(Size::relative(0.8).at_least(640.0))
                .size(Size::relative(0.1))
                .horizontal(|mut strip_v| {
                    strip_v.cell(|ui| {
                        // ui.painter().rect_stroke(
                        //     ui.available_rect_before_wrap(),
                        //     0.0,
                        //     Stroke::new(1.0, Color32::LIGHT_BLUE),
                        // );
                    });
                    strip_v.strip(|rows_builder| {
                        rows_builder
                            // .size(Size::relative(0.9).at_least(480.0).at_most(640.0))
                            // .size(Size::relative(0.1).at_least(12.0).at_most(24.0))
                            .size(Size::relative(0.9).at_least(540.0))
                            .size(Size::relative(0.1).at_most(30.0))
                            .vertical(|mut strip| {
                                strip.cell(|ui| {
                                    self.draw_content_scroll_area(ui);
                                    // ui.painter().rect_stroke(
                                    //     ui.available_rect_before_wrap(),
                                    //     0.0,
                                    //     Stroke::new(1.0, Color32::LIGHT_BLUE),
                                    // );
                                    // self.draw_content_scroll_area(ui);
                                    // egui::ScrollArea::vertical()
                                    //     .enable_scrolling(true)
                                    //     .hscroll(true)
                                    //     .stick_to_bottom(true)
                                    //     .show(ui, |ui| {
                                    //         ui.label(RichText::new("hello").color(Color32::RED));
                                    //         ui.label(RichText::new("world").color(Color32::BLUE));
                                    //     });
                                });
                                strip.strip(|strip_h| {
                                    strip_h
                                        .size(Size::relative(0.8))
                                        .size(Size::relative(0.1))
                                        .size(Size::relative(0.1))
                                        .horizontal(|mut strip| {
                                            strip.cell(|_| {});
                                            strip.cell(|ui| {
                                                let size = ui.available_rect_before_wrap().size();

                                                if ui
                                                    .add_sized(
                                                        ui.available_size_before_wrap(),
                                                        Button::new("hello").rounding(5.0),
                                                    )
                                                    .clicked()
                                                {
                                                    self.get_hello_service().say_hello();
                                                    info!("saying hello");
                                                };
                                            });
                                            strip.cell(|ui| {
                                                ui.label("right");
                                            });
                                        });
                                });
                            });
                    });
                    strip_v.cell(|ui| {
                        // ui.painter().rect_stroke(
                        //     ui.available_rect_before_wrap(),
                        //     0.0,
                        //     Stroke::new(1.0, Color32::LIGHT_BLUE),
                        // );
                    });
                });
        });
    }

    pub fn draw_content_scroll_area(&mut self, ui: &mut Ui) {
        let area_color = if ui.visuals().dark_mode {
            Color32::from_gray(40)
        } else {
            Color32::GRAY
        };
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.painter()
                .rect_filled(ui.available_rect_before_wrap(), 0.0, area_color);
            // ui.label(RichText::new("hello").color(Color32::RED));
            // ui.label(RichText::new("world").color(Color32::BLUE));
        });
    }
}
