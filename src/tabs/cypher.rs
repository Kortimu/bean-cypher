use crate::decode;
use crate::encode;
use crate::hash_convert::hash_conversions::get_default_hash;
use crate::ErrorType;
use crate::TestApp;

pub fn show_main_menu(ui: &mut egui::Ui, app: &mut TestApp) {
    ui.heading("CYPHER!!!!");

    ui.horizontal(|ui| {
        ui.set_height(150.0);
        egui::ScrollArea::vertical()
            .max_height(150.0)
            .show(ui, |ui| {
                ui.add_sized(
                    [ui.available_width() - 50.0, ui.available_height()],
                    egui::TextEdit::multiline(&mut app.input)
                        .hint_text("come the fuck on type some shit ya wanker"),
                );
            });

        ui.vertical(|ui| {
            if ui
                .add_sized(
                    [45.0, 73.0],
                    egui::Button::new(egui::RichText::new("⊗").size(35.0)),
                )
                .clicked()
            {
                app.input = String::new();
            }
            if ui
                .add_sized(
                    [45.0, 73.0],
                    egui::Button::new(egui::RichText::new("📂").size(35.0)),
                )
                .clicked()
            {
                app.active_error = Some(ErrorType::Wip);
            }
        });
    });

    ui.columns(2, |cols| {
        if cols[0].button("encode").clicked() {
            if app.input == String::new() {
                app.active_error = Some(ErrorType::EmptyInput);
            } else {
                app.output = encode::run(&app.input, &get_default_hash());

                app.output_shown = true;
            }
        }
        if cols[1].button("decode").clicked() {
            if app.input == String::new() {
                app.active_error = Some(ErrorType::EmptyInput);
            } else {
                if app.set_lowoutput {
                    app.output = decode::run(&app.input, &get_default_hash())
                        .unwrap()
                        .0
                        .to_lowercase();
                } else {
                    app.output = decode::run(&app.input, &get_default_hash()).unwrap().0;
                }
                app.output_shown = true;
            }
        }
    });

    if app.output_shown {
        show_output(ui, app, app.output.clone());
    }

    if app.active_error.is_some() {
        show_error(ui, app);
    }
}

fn show_error(ui: &egui::Ui, app: &mut TestApp) {
    let error_text = match app.active_error {
        Some(ErrorType::EmptyInput) => "maybe enter a fucking letter in dumbass bloke",
        Some(ErrorType::_FailedFile) => "cringe file moment",
        Some(ErrorType::_DecodingInputLacksInfo) => {
            "decoding error: maybe check what you want me to decode ya closeted fuck"
        }
        Some(ErrorType::Wip) => "i haven't implemented this yet nitwit",
        None => "IF YOU SEE THIS SOMETHING HAS GONE SEVERELY FUCKED",
    };

    egui::Modal::new(egui::Id::new("whoopsie_daisie")).show(ui, |ui| {
        ui.label(error_text);
        if ui.button("aight got it bud").clicked() {
            app.active_error = None;
        }
    });
}

fn show_output(ui: &egui::Ui, app: &mut TestApp, output: String) {
    egui::Modal::new(egui::Id::new("whoopsie_daisie")).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label("look at this shit");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🗙").clicked() {
                    app.output_shown = false;
                }
            });
        });
        ui.add_space(4.0);
        egui::ScrollArea::vertical()
            .max_height(250.0)
            .show(ui, |ui| {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.label(output.clone());
                });
            });
        ui.add_space(4.0);
        ui.columns(2, |cols| {
            if cols[0].button("copy").clicked() {
                cols[0].copy_text(output);
                app.output_shown = false;
            }
            if cols[1].button("save").clicked() {
                app.output_shown = false;
            }
        });
    });
}
