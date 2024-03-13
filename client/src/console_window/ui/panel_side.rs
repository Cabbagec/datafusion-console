use egui::RichText;
use log::info;

use crate::custom_widgets;

use super::ConsoleApp;

impl ConsoleApp {
    pub fn draw_console_side_panel_in_ui(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::left("menu_panel")
            .resizable(false)
            .max_width(180.0)
            .show_inside(ui, |ui| {
                ui.label(RichText::new("Select context"));
                ui.vertical_centered_justified(|ui| {
                    // ui.columns(2, |cols| {
                    //     cols[0].add(custom_widgets::searchable_dropdown::DropDownBox::from_iter(
                    //         vec!["A", "B", "c"],
                    //         "runtime_context",
                    //         &mut select,
                    //         |ui, item| ui.selectable_label(false, item),
                    //     ));
                    //     cols[1].add(egui::Button::new("hi"));
                    // });
                    let status = self.get_status();
                    let all_ctx_names = status.context_names.borrow().clone();
                    // let mut current_ctx_mut = status.current_context_name.borrow_mut();
                    ui.horizontal(|ui| {
                        let size = ui.available_size_before_wrap();
                        let r = ui.add_sized(
                            [size.x / 6.0 * 5.0, size.y],
                            custom_widgets::searchable_dropdown::DropDownBox::from_iter(
                                all_ctx_names,
                                "runtime_context",
                                &mut *(status.edit_ctx_name.borrow_mut()),
                                |ui, item| ui.selectable_label(false, item),
                            )
                            .hint_text("Default"),
                        );
                        let mut edit = status.edit_ctx_name.borrow_mut();
                        let mut ctxes = status.context_names.borrow_mut();
                        let mut current_mut = status.current_context_name.borrow_mut();

                        // list content is selected, set it as current
                        if r.changed() && ctxes.contains(&*edit) {
                            info!("changed, edit: {edit}");
                            current_mut.replace(edit.clone());
                        }

                        // activated, clear current content in edit
                        if r.gained_focus() {
                            edit.clear();
                            info!("gained focus, clear edit!");
                        }

                        // "+" or enter is pressed, add non-duplicate edit content to context_names and set as current
                        if (ui
                            .add_sized([size.x / 6.0, size.y], egui::Button::new("+"))
                            .clicked()
                            || (r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
                            && !ctxes.contains(&*edit)
                        {
                            if edit.is_empty() {
                                *edit = "Default".to_string();
                            }
                            let new_ctx = edit.clone();
                            if ctxes.insert(new_ctx.clone()) {
                                info!("insert new ctx: {new_ctx}");
                                current_mut.replace(new_ctx.clone());
                            } else {
                                info!("already have ctx: {new_ctx}");
                            };
                        };

                        // leave edit, restore edit content with current context
                        if r.lost_focus() && !ctxes.contains(&*edit) {
                            info!("lost focus, set edit to: {current_mut:?}");
                            *edit = current_mut.clone().unwrap_or_default();
                            info!("edit ctx: {edit}");
                        }
                    });
                    // egui::Grid::new("rt_control_grid")
                    //     .num_columns(2)
                    //     .min_col_width(12.0)
                    //     .striped(true)
                    //     .show(ui, |ui| {
                    //         ui.add(egui::Label::new("Select context: "));
                    //         ui.button("hi");
                    //         ui.add(custom_widgets::toggle_ui::toggle_switch(
                    //             &mut self.get_status().pause_server_yields.borrow_mut(),
                    //         ));
                    //         ui.end_row();
                    //
                    //         ui.label("2");
                    //         custom_widgets::toggle_ui::toggle_ui(
                    //             ui,
                    //             &mut self.get_status().pause_server_yields.borrow_mut(),
                    //         );
                    //         ui.end_row();
                    //
                    //         ui.label("3");
                    //         custom_widgets::toggle_ui::toggle_ui(
                    //             ui,
                    //             &mut self.get_status().pause_server_yields.borrow_mut(),
                    //         );
                    //         ui.end_row();
                    //     });
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
