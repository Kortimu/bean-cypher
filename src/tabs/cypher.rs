use crate::TestApp;
use crate::ErrorType;

pub fn show_main_menu(ui: &mut egui::Ui, app: &mut TestApp) {
    ui.heading("CYPHER!!!!");

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add(egui::TextEdit::multiline(&mut app.input));
    });

    ui.columns(2, |cols| {
        if cols[0].button("encode").clicked() {
            if app.input == String::new() {
                app.active_error = Some(ErrorType::EmptyInput);
            } else {
                app.output_shown = true;
            }
        }
        if cols[1].button("decode").clicked() {
            if app.input == String::new() {
                app.active_error = Some(ErrorType::EmptyInput);
            } else {
                app.output_shown = true;
            }
        }
    });

    if app.output_shown {
        show_output(ui, app);
    }

    if app.active_error.is_some() {
        show_error(ui, app);
    }
}

fn show_error(ui: &mut egui::Ui, app: &mut TestApp) {
    let error_text = match app.active_error {
        Some(ErrorType::EmptyInput) => "maybe enter a fucking letter in dumbass bloke",
        Some(ErrorType::_FailedFile) => "cringe file moment",
        None => "IF YOU SEE THIS SOMETHING HAS GONE SEVERELY FUCKED"
    };
    
    egui::Modal::new(egui::Id::new("whoopsie_daisie"))
        .show(ui, |ui| {
            ui.label(error_text);
            if ui.button("OK").clicked() {
                app.active_error = None;
            }
        });
}

fn show_output(ui: &mut egui::Ui, app: &mut TestApp) {
    egui::Modal::new(egui::Id::new("whoopsie_daisie"))
        .show(ui, |ui| {
            ui.label("look at this shit:");
            egui::Frame::group(ui.style()).show(ui, |ui| {
                // FIXME: reminder that here goes the encoded stuffs
                ui.label(app.input.clone());
            });
            // ui.columns(2, |_cols| {
                if ui.button("copy").clicked() {
                    app.output_shown = false;
                }
                if ui.button("save").clicked() {
                    app.output_shown = false;
                }
            // });
        });
}