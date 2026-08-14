use crate::decode;
use crate::encode;
use crate::get_app_version;
use crate::hash_convert::hash_conversions::get_default_hash;
use crate::ErrorType;
use crate::TestApp;
use egui_i18n::tr;

#[allow(clippy::too_many_lines)]
pub fn show_main_menu(ui: &mut egui::Ui, app: &mut TestApp) {
    let is_mobile = ui.ctx().content_rect().width() < 600.0;
    ui.horizontal(|ui| {
        ui.add(
            egui::Image::new(
                egui::include_image!("../../assets/beano.png"), // nice
            )
            .fit_to_exact_size(egui::vec2(69.0, 69.0)),
        );
        ui.allocate_ui_with_layout(
            egui::vec2(150.0, 69.0),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                //ui.heading("bean cypher v0.7.0");
                ui.heading(format!(
                    "{} v{}",
                    tr!("app_name"),
                    env!("CARGO_PKG_VERSION")
                ));
                // TODO: this is where a random quote could go hard
                ui.label(egui::RichText::new("\"what a bullshit\"").italics());
            },
        );

        // this only shows up when a newer version is noticed.
        // yes, this is my update notification system :3
        if !is_mobile
            && app.newest_version_found.unwrap_or((0, 0)) > get_app_version().unwrap_or((0, 0))
        {
            show_update_popup(ui, app);
        }
    });

    // this only shows up when a newer version is noticed.
    // yes, this is my update notification system :3
    if is_mobile && app.newest_version_found.unwrap_or((0, 0)) > get_app_version().unwrap_or((0, 0))
    {
        show_update_popup(ui, app);
    }

    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.set_height(150.0);
        egui::ScrollArea::vertical()
            .max_height(150.0)
            .show(ui, |ui| {
                ui.add_sized(
                    [ui.available_width() - 50.0, ui.available_height()],
                    egui::TextEdit::multiline(&mut app.input).hint_text(tr!("cypher_input_hint")),
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
            // TODO: add paste button (for mobile users)
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
        if cols[0]
            .add_sized(
                [cols[0].available_width(), 25.0],
                egui::Button::new(tr!("cypher_encode")),
            )
            .clicked()
        {
            if app.input == String::new() {
                app.active_error = Some(ErrorType::EmptyInput);
            } else {
                app.newest_version_found = None;
                app.output_warning = String::new();

                app.output = encode::run(
                    &app.input,
                    &get_default_hash(),
                    app.set_edge,
                    app.set_edge_link.clone(),
                );
                app.output_shown = true;
            }
        }

        if cols[1]
            .add_sized(
                [cols[1].available_width(), 25.0],
                egui::Button::new(tr!("cypher_decode")),
            )
            .clicked()
        {
            if app.input == String::new() {
                app.active_error = Some(ErrorType::EmptyInput);
            } else {
                // would make this make sense but i'll do that when i rewrite decode.rs
                let response = decode::run(&app.input.clone(), &get_default_hash(), Some(app))
                    .unwrap_or_else(|_| (String::new(), String::from("no worky :[")));
                app.output_warning = response.1;
                if app.set_lowoutput {
                    app.output = response.0.to_lowercase();
                } else {
                    app.output = response.0;
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

fn show_update_popup(ui: &mut egui::Ui, app: &mut TestApp) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::new(
                        egui::include_image!("../../assets/warning.png")
                    ).fit_to_exact_size(egui::vec2(30.0, 30.0))
                );
                ui.vertical(|ui| {
                    //ui.label(egui::RichText::new(format!("based on prior decodings, the newest version of the program is v{}.{}.x. please update to the newest version, if possible!!!", app.newest_version_found.unwrap_or((0, 0)).0, app.newest_version_found.unwrap_or((0, 0)).1)).size(12.0));
                    ui.label(egui::RichText::new(tr!("cypher_update", { major: app.newest_version_found.unwrap_or((0, 0)).0, minor: app.newest_version_found.unwrap_or((0,0)).1 })));
                });
            });

            ui.columns(2, |cols| {
                if cols[0].add_sized(
                    [cols[0].available_width(), 20.0],
                    egui::Button::new(tr!("cypher_update_yes"))
                ).clicked() {
                    cols[0].ctx().open_url(egui::OpenUrl {
                        url: "https://github.com/Kortimu/bean-cypher/releases".to_owned(),
                        new_tab: true
                    });
                }

                if cols[1].add_sized(
                    [cols[1].available_width(), 20.0],
                    egui::Button::new(tr!("cypher_update_no"))
                ).clicked() {
                    app.newest_version_found = None;
                }
            });

        });
    });
}

fn show_error(ui: &egui::Ui, app: &mut TestApp) {
    let error_text = match app.active_error {
        Some(ErrorType::EmptyInput) => tr!("error_empty"),
        Some(ErrorType::_FailedFile) => tr!("error_file"),
        Some(ErrorType::_DecodingInputLacksInfo) => tr!("error_decode"),
        Some(ErrorType::Wip) => tr!("error_wip"),
        None => tr!("error_what"),
    };

    egui::Modal::new(egui::Id::new("whoopsie_daisie")).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add(
                egui::Image::new(egui::include_image!("../../assets/warning.png"))
                    .fit_to_exact_size(egui::vec2(25.0, 25.0)),
            );
            ui.heading(tr!("error"));
        });
        ui.label(error_text);
        ui.add_space(4.0);
        if ui
            .add_sized(
                [ui.available_width(), 25.0],
                egui::Button::new(tr!("error_ok")),
            )
            .clicked()
        {
            app.active_error = None;
        }
    });
}

fn show_output(ui: &egui::Ui, app: &mut TestApp, output: String) {
    egui::Modal::new(egui::Id::new("whoopsie_daisie")).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(tr!("output_label"));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🗙").clicked() {
                    app.output_shown = false;
                }
            });
        });
        ui.add_space(4.0);

        if app.output_warning != String::new() {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::new(egui::include_image!("../../assets/warning.png"))
                            .fit_to_exact_size(egui::vec2(45.0, 45.0)),
                    );
                    ui.vertical(|ui| {
                        ui.label(app.output_warning.clone());
                    });
                });
            });
            ui.add_space(4.0);
        }

        egui::ScrollArea::vertical()
            .max_height(250.0)
            .show(ui, |ui| {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.label(output.clone());
                });
            });
        ui.add_space(4.0);
        ui.columns(2, |cols| {
            if cols[0]
                .add_sized(
                    [cols[0].available_width(), 25.0],
                    egui::Button::new(tr!("output_copy")),
                )
                .clicked()
            {
                cols[0].copy_text(output);
                app.output_shown = false;
            }
            if cols[1]
                .add_sized(
                    [cols[0].available_width(), 25.0],
                    egui::Button::new(tr!("output_save")),
                )
                .clicked()
            {
                app.output_shown = false;
            }
        });
    });
}
