use egui::RichText;

use crate::custom_widgets;

use super::ConsoleApp;

impl ConsoleApp {
    pub fn draw_side_panels_in_ui(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::left("menu_panel")
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.label(RichText::new("Server Controls").heading());
                ui.vertical_centered_justified(|ui| {
                    egui::Grid::new("rt_control_grid")
                        .num_columns(2)
                        .min_col_width(12.0)
                        .striped(true)
                        .show(ui, |ui| {
                            ui.add(egui::Label::new("pause server yieldings control"));
                            ui.add(custom_widgets::toggle_ui::toggle_switch(
                                &mut self.get_status().pause_server_yields.borrow_mut(),
                            ));
                            ui.end_row();

                            ui.label("2");
                            custom_widgets::toggle_ui::toggle_ui(
                                ui,
                                &mut self.get_status().pause_server_yields.borrow_mut(),
                            );
                            ui.end_row();

                            ui.label("3");
                            custom_widgets::toggle_ui::toggle_ui(
                                ui,
                                &mut self.get_status().pause_server_yields.borrow_mut(),
                            );
                            ui.end_row();
                        });
                });
            });
        egui::SidePanel::right("status_panel")
            .resizable(false)
            .min_width(80.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Status");
                });
            });
    }
}
