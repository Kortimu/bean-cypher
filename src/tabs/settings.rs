use crate::{TestApp, toggle};

pub fn show_settings(ui: &mut egui::Ui, app: &mut TestApp) {
    ui.heading("SETTINGS!!!!");
    ui.horizontal(|ui| {
        ui.add(toggle(&mut app.set_lowoutput));
        ui.label("lowercase output");
    });
}