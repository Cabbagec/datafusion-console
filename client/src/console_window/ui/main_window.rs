use eframe::{Frame, Storage};
use egui::Context;

use crate::console_window::app::ConsoleApp;
use crate::status::Mode;

impl eframe::App for ConsoleApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.draw_top_menu_in_ctx(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            let connected = self.get_status().connected;
            let mode = self.get_status().mode.borrow().clone();
            // ui.add_enabled_ui(connected, |ui| {
            ui.add_enabled_ui(true, |ui| match mode {
                Mode::Console => {
                    self.draw_console_side_panel_in_ui(ui);
                    self.draw_console_center_panel_in_ui(ui);
                }
                Mode::Monitor => {}
            });
        });
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        let clean = self.clone_clean_app();
        eframe::set_value(_storage, &self.get_label(), &clean);
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
