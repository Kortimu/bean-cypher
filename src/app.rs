use std::fs;
use std::io::Read;

use crate::decode;
use crate::encode;
use crate::ErrorState;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BeanCypher {
    #[serde(skip)]
    input: String,
    #[serde(skip)]
    output: String,
    #[serde(skip)]
    show_settings: bool,
    #[serde(skip)]
    show_credits: bool,
    #[serde(skip)]
    current_error: ErrorState,

    set_lowercase: bool,
    // set_lang: String
}

impl Default for BeanCypher {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            show_settings: false,
            show_credits: false,
            current_error: ErrorState::None,
            set_lowercase: false,
        }
    }
}

impl BeanCypher {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let old_style = cc.egui_ctx.style().as_ref().clone();
        let old_spacing = cc.egui_ctx.style().spacing.clone();
        cc.egui_ctx.set_style(egui::Style {
            spacing: egui::style::Spacing {
                scroll: egui::style::ScrollStyle::solid(),
                ..old_spacing
            },
            ..old_style
        });
        egui_extras::install_image_loaders(&cc.egui_ctx);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

impl eframe::App for BeanCypher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            display_menu_bar(self, ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            display_central_panel(self, ctx, ui);
        });

        if self.show_settings {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("settings"),
                egui::ViewportBuilder::default()
                    .with_title("Settings")
                    .with_maximize_button(false)
                    .with_inner_size([400.0, 200.0]),
                |ctx, _class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Settings");
                        ui.label("(Only theme selection does not get saved.)");

                        ui.separator();

                        egui::Grid::new("settings_grid")
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Theme");
                                egui::global_dark_light_mode_buttons(ui);
                                ui.end_row();

                                ui.label("Lowercase output");
                                ui.checkbox(&mut self.set_lowercase, "");
                                ui.end_row();

                                // just did some testing, leaving here for later

                                // ui.label("Language");
                                // egui::ComboBox::from_label("Pick a language :]")
                                //     .selected_text(format!("{:?}", self.set_lang))
                                //     .show_ui(ui, |ui| {
                                //         ui.selectable_value(&mut self.set_lang, String::from("English"), "English (default)");
                                //         ui.selectable_value(&mut self.set_lang, String::from("Latvian"), "Latvian");
                                //     }
                                // );
                            });
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.show_settings = false;
                    }
                },
            );
        }

        if self.show_credits {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("credits"),
                egui::ViewportBuilder::default()
                    .with_title("Credits")
                    .with_maximize_button(false)
                    .with_inner_size([250.0, 200.0]),
                |ctx, _class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.set_height(100.0);

                            ui.add(egui::Image::new(egui::include_image!("../assets/bean.png")));
                            ui.vertical(|ui| {
                                ui.add_space(30.0);
                                ui.heading("Bean Cypher");
                                ui.label(format!("Alpha v{}-dev", env!("CARGO_PKG_VERSION")));
                            });
                        });

                        ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));
                        ui.horizontal(|ui| {
                            ui.label("Creator:");
                            ui.hyperlink_to("Kortimu", "https://kortimu.github.io");
                        });
                        ui.horizontal(|ui| {
                            ui.label("Chief Bean Officer:");
                            ui.hyperlink_to("Bean Man", "https://twitch.tv/beandhd");
                        });
                        ui.hyperlink_to("Source code", "https://github.com/Kortimu/bean-cypher");
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.show_credits = false;
                    }
                },
            );
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn display_menu_bar(app: &mut BeanCypher, ui: &mut egui::Ui) {
    egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Encode from file...").clicked() {
                match rfd::FileDialog::new()
                    .add_filter("Text", &["txt"])
                    .pick_file()
                {
                    Some(file_path) => {
                        let potential_file = std::fs::File::open(file_path);

                        if potential_file.is_ok() {
                            let mut file = potential_file.expect("ah fuck");
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).expect("ah fuck");
                            app.output = encode::run(&contents);
                            app.current_error = ErrorState::None;
                        } else {
                            app.current_error = ErrorState::Error(
                                "Error: Failed to parse selected file.".to_string(),
                            );
                        }
                    }
                    None => {
                        app.current_error = ErrorState::Error(
                            "File error: Failed to select a text file.".to_string(),
                        );
                    }
                }
                ui.close_menu();
            }
            if ui.button("Decode from file...").clicked() {
                match rfd::FileDialog::new()
                    .add_filter("Text", &["txt"])
                    .pick_file()
                {
                    Some(file_path) => {
                        let potential_file = std::fs::File::open(file_path);

                        if potential_file.is_ok() {
                            let mut file = potential_file.expect("ah fuck");
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).expect("ah fuck");
                            match decode::run(&contents) {
                                Ok(result) => {
                                    app.output = result.0;
                                    app.current_error = ErrorState::Warning(result.1);
                                }
                                Err(error) => app.current_error = error,
                            };
                            app.current_error = ErrorState::None;
                        } else {
                            app.current_error = ErrorState::Error(
                                "Error: Failed to parse selected file.".to_string(),
                            );
                        }
                    }
                    None => {
                        app.current_error = ErrorState::Error(
                            "File error: Failed to select a text file.".to_string(),
                        );
                    }
                }
                ui.close_menu();
            }
            // reminder: this will probably be only on windows
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }
        });

        if ui.button("Settings").clicked() {
            app.show_settings = true;
        }
        if ui.button("Credits").clicked() {
            app.show_credits = true;
        }
    });
}

fn display_central_panel(app: &mut BeanCypher, ctx: &egui::Context, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.set_height(55.0);

            ui.add(egui::Image::new(egui::include_image!("../assets/bean.png")));
            ui.vertical(|ui| {
                ui.add_space(5.0);
                ui.heading("Bean Cypher");
                ui.label(format!("Alpha v{}-dev", env!("CARGO_PKG_VERSION")));
            });

            display_info_bar(app, ui);
        });

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_sized(
                [ctx.input(|i| i.screen_rect().width() - 15.0), 100.0],
                egui::TextEdit::multiline(&mut app.input)
                    .hint_text("Input goes here...")
                    .clip_text(true),
            );
        });

        ui.horizontal(|ui| {
            if ui.button("Encode text").clicked() {
                app.output = encode::run(&app.input);
                app.current_error = ErrorState::None;
            }
            if ui.button("Decode text").clicked() {
                match decode::run(&app.input) {
                    Ok(result) => {
                        app.output = result.0;
                        if result.1.is_empty() {
                            app.current_error = ErrorState::None;
                        } else {
                            app.current_error = ErrorState::Warning(result.1);
                        }
                    }
                    Err(error) => app.current_error = error,
                };
                if app.set_lowercase {
                    app.output = app.output.to_lowercase();
                }
            }
        });

        ui.separator();

        ui.label(app.output.to_string());

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Copy to clipboard").clicked() {
                ctx.copy_text(app.output.clone());
            }
            if ui.button("Save output as...").clicked() {
                let dialog = rfd::FileDialog::new()
                    .set_file_name("output.txt")
                    .add_filter("Text", &["txt"])
                    .save_file();

                if let Some(file) = dialog {
                    let _ = fs::write(file, app.output.clone());
                }
            }
        });
    });
}

fn display_info_bar(app: &BeanCypher, ui: &mut egui::Ui) {
    egui::Frame::none()
        .fill(match app.current_error {
            ErrorState::Error(_) => egui::Color32::from_hex("#cf103190")
                .expect("Error: Faulty hex code value for warning."),
            ErrorState::Warning(_) => egui::Color32::from_hex("#d0b010")
                .expect("Error: Faulty hex code value for warning."),
            ErrorState::None => {
                egui::Color32::from_hex("#0000").expect("Error: Faulty hex code value for warning.")
            }
        })
        .rounding(egui::Rounding {
            nw: 5.0,
            ne: 5.0,
            sw: 5.0,
            se: 5.0,
        })
        .inner_margin(egui::Margin {
            left: 10.0,
            top: 5.0,
            bottom: 5.0,
            right: 10.0,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.scope(|ui| {
                if ui.style().visuals.dark_mode {
                    match app.current_error {
                        ErrorState::Error(_) => {
                            ui.style_mut().visuals.override_text_color = Some(
                                egui::Color32::from_hex("#dfdfdf")
                                    .expect("Error: Faulty hex code value for warning."),
                            );
                        }
                        ErrorState::Warning(_) => {
                            ui.style_mut().visuals.override_text_color = Some(
                                egui::Color32::from_hex("#303030")
                                    .expect("Error: Faulty hex code value for warning."),
                            );
                        }
                        ErrorState::None => (),
                    }
                }

                match app.current_error {
                    ErrorState::Error(_) => ui.add(
                        egui::Image::new(egui::include_image!("../assets/error.png"))
                            .max_height(30.0),
                    ),
                    ErrorState::Warning(_) => ui.add(
                        egui::Image::new(egui::include_image!("../assets/warning.png"))
                            .max_height(30.0),
                    ),
                    // FIXME: janky fucked up workaround that i'd prefer not exist
                    ErrorState::None => ui.add(
                        egui::Image::new(egui::include_image!("../assets/error.png"))
                            .max_height(0.0),
                    ),
                };

                ui.label(app.current_error.clone().into_string());
            });
        });
}
