use crate::{load_flag, toggle, TestApp};
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

            ui.horizontal(|ui| {
                ui.label(tr!("set_lang"));
                let mut current_lang = egui_i18n::get_language();
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        egui::ComboBox::from_id_salt("box_lang")
                            .selected_text(tr!("lang_name"))
                            .show_ui(ui, |ui| {
                                for lang in egui_i18n::languages() {
                                    let flag = load_flag(ui.ctx(), &lang);
                                    ui.horizontal(|ui| {
                                        ui.add(egui::Image::new(&flag).max_width(16.0));
                                        if ui
                                            .selectable_value(
                                                &mut current_lang,
                                                lang.clone(),
                                                &lang,
                                            )
                                            .clicked()
                                        {
                                            egui_i18n::set_language(&lang);
                                            app.set_lang = String::from(&lang);
                                        }
                                    });
                                }
                            });
                    },
                );
            });

            ui.horizontal(|ui| {
                ui.label(tr!("set_theme"));
                // i could set the theme every frame but eh, why not optimize
                let theme_setting_of_before = app.set_theme;
                app.set_theme.radio_buttons(ui);
                if theme_setting_of_before != app.set_theme {
                    ui.ctx().options_mut(|o| {
                        o.theme_preference = app.set_theme;
                    });
                }
            });
        },
    );
}
