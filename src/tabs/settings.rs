use crate::{toggle, TestApp};
use egui_i18n::tr;

pub fn show_settings(ui: &mut egui::Ui, app: &mut TestApp) {
    ui.allocate_ui_with_layout(
        ui.available_size(),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            ui.set_max_width(300.0);

            ui.horizontal(|ui| {
                ui.label(tr!("set_lowercase"));
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        ui.add(toggle(&mut app.set_lowoutput));
                    },
                );
            });
        },
    );
}
