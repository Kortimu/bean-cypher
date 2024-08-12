use std::collections::HashMap;
use std::fs;
use std::io::Read;

use crate::decode;
use crate::encode;
use crate::hash_convert;
use crate::hash_convert::hash_conversions::get_default_hash;
use crate::ErrorState;
use crate::Language;
use crate::ENGLISH;
use crate::LATVIAN;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[allow(clippy::struct_excessive_bools)]
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
    set_custom_cypher: bool,
    set_cypher: (String, HashMap<usize, String>),
    #[serde(skip)]
    set_silly: bool,
    // FIXME: i'd rather not have this not saved but lifetimes are bullying me so i will accept a life without saved language preferences :[
    #[serde(skip)]
    set_language: Language,
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
            set_custom_cypher: false,
            set_cypher: (
                "default :]".to_string(),
                hash_convert::hash_conversions::get_default_hash(),
            ),
            set_silly: false,
            set_language: ENGLISH,
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
            display_settings_window(self, ctx);
        }

        if self.show_credits {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("credits"),
                egui::ViewportBuilder::default()
                    .with_title(self.set_language.menu_credits)
                    .with_maximize_button(false)
                    .with_inner_size([350.0, 500.0]),
                |ctx, _class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.set_height(100.0);

                            ui.add(egui::Image::new(egui::include_image!("../assets/bean.png")));
                            ui.vertical(|ui| {
                                ui.add_space(30.0);
                                ui.heading(self.set_language.app_name);
                                ui.label(format!("Alpha v{}-dev", env!("CARGO_PKG_VERSION")));
                            });
                        });

                        ui.label(format!(
                            "{}: {}",
                            self.set_language.cred_version,
                            env!("CARGO_PKG_VERSION")
                        ));
                        ui.horizontal(|ui| {
                            ui.label(format!("{}:", self.set_language.cred_creator));
                            ui.hyperlink_to("Kortimu", "https://kortimu.github.io");
                        });
                        ui.horizontal(|ui| {
                            ui.label(format!("{}:", self.set_language.cred_cbo));
                            ui.hyperlink_to("Bean Man", "https://twitch.tv/beandhd");
                        });
                        ui.horizontal(|ui| {
                            ui.label(format!("{}:", self.set_language.cred_fav_station));
                            ui.hyperlink_to("Gensokyo Radio :]", "https://gensokyoradio.net");
                        });
                        ui.add_space(5.0);
                        ui.label(self.set_language.cred_sanity);
                        ui.add_space(10.0);
                        ui.hyperlink_to(
                            self.set_language.cred_source,
                            "https://github.com/Kortimu/bean-cypher",
                        );
                        ui.hyperlink_to(
                            self.set_language.cred_website,
                            "https://kortimu.github.io/bean-cypher",
                        );
                        ui.add_space(10.0);
                        ui.label(format!("{}:", self.set_language.cred_funny_img));
                        ui.image(egui::include_image!("../assets/funny_image.png"));
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
        ui.menu_button(app.set_language.menu_file, |ui| {
            if ui.button(app.set_language.menu_file_encode).clicked() {
                match rfd::FileDialog::new()
                    .add_filter(app.set_language.text, &["txt"])
                    .pick_file()
                {
                    Some(file_path) => {
                        let potential_file = std::fs::File::open(file_path);

                        if potential_file.is_ok() {
                            let mut file = potential_file.expect("ah fuck");
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).expect("ah fuck");
                            app.output = encode::run(&contents, &get_hash(app));
                            app.current_error = ErrorState::None;
                        } else {
                            app.current_error =
                                ErrorState::Error(app.set_language.err_file_parse.to_string());
                        }
                    }
                    None => {
                        app.current_error =
                            ErrorState::Error(app.set_language.err_file_select.to_string());
                    }
                }
                ui.close_menu();
            }
            if ui.button(app.set_language.menu_file_decode).clicked() {
                match rfd::FileDialog::new()
                    .add_filter(app.set_language.text, &["txt"])
                    .pick_file()
                {
                    Some(file_path) => {
                        let potential_file = std::fs::File::open(file_path);

                        if potential_file.is_ok() {
                            let mut file = potential_file.expect("ah fuck");
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).expect("ah fuck");
                            match decode::run(&contents, &get_hash(app)) {
                                Ok(result) => {
                                    app.output = result.0;
                                    app.current_error = ErrorState::Warning(result.1);
                                }
                                Err(error) => app.current_error = error,
                            };
                            app.current_error = ErrorState::None;
                        } else {
                            app.current_error =
                                ErrorState::Error(app.set_language.err_file_parse.to_string());
                        }
                    }
                    None => {
                        app.current_error =
                            ErrorState::Error(app.set_language.err_file_select.to_string());
                    }
                }
                ui.close_menu();
            }
            // reminder: this will probably be only on windows
            if ui.button(app.set_language.menu_file_quit).clicked() {
                std::process::exit(0);
            }
        });

        if ui.button(app.set_language.menu_settings).clicked() {
            app.show_settings = true;
        }
        if ui.button(app.set_language.menu_credits).clicked() {
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
                ui.heading(app.set_language.app_name);
                ui.label(format!("Alpha v{}-dev", env!("CARGO_PKG_VERSION")));
            });

            display_info_bar(app, ui);
        });

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_sized(
                [ctx.input(|i| i.screen_rect().width() - 15.0), 100.0],
                egui::TextEdit::multiline(&mut app.input)
                    .hint_text(app.set_language.hint_input)
                    .clip_text(true),
            );
        });

        ui.horizontal(|ui| {
            if ui.button(app.set_language.btn_encode).clicked() {
                app.output = encode::run(&app.input, &get_hash(app));
                app.current_error = ErrorState::None;
            }
            if ui.button(app.set_language.btn_decode).clicked() {
                match decode::run(&app.input, &get_hash(app)) {
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
            if ui.button(app.set_language.btn_copy).clicked() {
                ctx.copy_text(app.output.clone());
            }
            if ui.button(app.set_language.btn_save).clicked() {
                let dialog = rfd::FileDialog::new()
                    .set_file_name("output.txt")
                    .add_filter(app.set_language.text, &["txt"])
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

fn display_settings_window(app: &mut BeanCypher, ctx: &egui::Context) {
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("settings"),
        egui::ViewportBuilder::default()
            .with_title(app.set_language.menu_settings)
            .with_maximize_button(false)
            .with_inner_size([450.0, 300.0]),
        |ctx, _class| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(app.set_language.menu_settings);
                ui.label(app.set_language.settings_flavour);

                ui.separator();

                egui::Grid::new("settings_grid")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label(app.set_language.set_theme);
                        egui::global_dark_light_mode_buttons(ui);
                        ui.end_row();

                        ui.label(app.set_language.set_lowercase);
                        ui.checkbox(&mut app.set_lowercase, "");
                        ui.end_row();

                        ui.label(app.set_language.set_enable_cypher);
                        ui.checkbox(&mut app.set_custom_cypher, "");
                        ui.end_row();

                        ui.label(app.set_language.set_cypher);
                        if ui.button(app.set_language.set_cypher_import).clicked() {
                            match rfd::FileDialog::new()
                                .add_filter(app.set_language.text, &["txt"])
                                .pick_file()
                            {
                                Some(file_path) => {
                                    // FIXME: janky workaround i think? probably some cleverer way to do this but listen man it's late already
                                    let path = file_path.clone();
                                    match hash_convert::hash_conversions::file_to_hash(file_path) {
                                        Ok(result) => {
                                            match hash_convert::hash_conversions::validate_cypher(
                                                &result,
                                            ) {
                                                Some(()) => {
                                                    app.set_cypher.0 = path
                                                        .file_name()
                                                        .unwrap_or_else(|| {
                                                            std::ffi::OsStr::new("Name unknown.")
                                                        })
                                                        .to_str()
                                                        .unwrap_or("Name unknown.")
                                                        .to_string();

                                                    app.set_cypher.1 = result;
                                                    app.current_error = ErrorState::None;
                                                }
                                                None => {
                                                    app.current_error = ErrorState::Error(
                                                        app.set_language
                                                            .err_cypher_phrase
                                                            .to_string(),
                                                    );
                                                }
                                            }
                                        }
                                        Err(error) => app.current_error = error,
                                    }
                                }
                                None => {
                                    app.current_error = ErrorState::Error(
                                        app.set_language.err_file_location.to_string(),
                                    );
                                }
                            }
                        }
                        ui.label(app.set_cypher.0.clone());
                        ui.end_row();

                        ui.label(app.set_language.set_silly);
                        let silly_text: &str = if app.set_silly {
                            app.set_language.silly_true
                        } else {
                            app.set_language.silly_false
                        };

                        if ui.button(silly_text).clicked() {
                            app.set_silly = !app.set_silly;
                            if app.set_silly {
                                let mut fonts = egui::FontDefinitions::default();
                                fonts.font_data.insert(
                                    "Comic Sans".to_string(),
                                    egui::FontData::from_static(include_bytes!(
                                        "../assets/comic_sans.ttf"
                                    )),
                                );
                                fonts
                                    .families
                                    .entry(egui::FontFamily::Proportional)
                                    .or_default()
                                    .insert(0, "Comic Sans".to_string());
                                ctx.set_fonts(fonts);
                            } else {
                                ctx.set_fonts(egui::FontDefinitions::default());
                            }
                        }
                        ui.end_row();

                        ui.label(app.set_language.set_lang);
                        egui::ComboBox::from_id_source("box_lang")
                            .selected_text(format!("{:?}", app.set_language.lang_name))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut app.set_language,
                                    ENGLISH,
                                    ENGLISH.lang_name,
                                );
                                ui.selectable_value(
                                    &mut app.set_language,
                                    LATVIAN,
                                    LATVIAN.lang_name,
                                );
                            })
                    });
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                app.show_settings = false;
            }
        },
    );
}

fn get_hash(app: &BeanCypher) -> HashMap<usize, String> {
    if app.set_custom_cypher {
        app.set_cypher.1.clone()
    } else {
        get_default_hash()
    }
}
