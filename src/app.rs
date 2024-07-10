use std::io::Read;

use crate::decode;
use crate::encode;

#[derive(Default)]
pub struct BeanCypherApp {
    input: String,
    output: String,
    show_settings: bool,
    set_lowercase: bool,
    // set_lang: String
}

impl BeanCypherApp {
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
        Self::default()
    }
}

impl eframe::App for BeanCypherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Encode from file...").clicked() {
                        let file_path = rfd::FileDialog::new()
                            .add_filter("text", &["txt"])
                            .pick_file()
                            .unwrap();

                        let potential_file = std::fs::File::open(file_path);

                        if potential_file.is_ok() {
                            let mut file = potential_file.expect("ah fuck");
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).expect("ah fuck");
                            self.output = encode::run(&contents);
                        } else {
                            todo!();
                        }
                    }
                    if ui.button("Decode from file...").clicked() {
                        let file_path = rfd::FileDialog::new()
                            .add_filter("text", &["txt"])
                            .pick_file()
                            .expect("Error with selecting file. Behaviour will be implemented later.");

                        let potential_file = std::fs::File::open(file_path);

                        if potential_file.is_ok() {
                            let mut file = potential_file.expect("ah fuck");
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).expect("ah fuck");
                            self.output = decode::run(&contents);
                        } else {
                            todo!();
                        }
                    }
                });

                if ui.button("Open settings").clicked() {
                    self.show_settings = true;
                }
                // TODO: credits button
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.set_height(50.0);

                    ui.add(
                        egui::Image::new(egui::include_image!("../assets/bean.png"))
                            .max_width(50.0),
                    );
                    ui.vertical(|ui| {
                        ui.add_space(5.0);
                        ui.heading("Bean Cypher");
                        ui.label(format!("Alpha v{}-dev", env!("CARGO_PKG_VERSION")));
                    });
                });

                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        [ctx.input(|i| i.screen_rect().width() - 15.0), 100.0],
                        egui::TextEdit::multiline(&mut self.input)
                            .hint_text("Input goes here...")
                            .clip_text(true),
                    );
                });

                ui.horizontal(|ui| {
                    if ui.button("Encode text").clicked() {
                        self.output = encode::run(&self.input);
                    }
                    if ui.button("Decode text").clicked() {
                        self.output = decode::run(&self.input);
                        if self.set_lowercase {
                            self.output = self.output.to_lowercase();
                        }
                    }
                });

                ui.separator();

                ui.label(self.output.to_string());

                ui.separator();

                if ui.button("Copy to clipboard").clicked() {
                    ctx.copy_text(self.output.clone());
                }
            });
        });

        if self.show_settings {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("settings"),
                egui::ViewportBuilder::default()
                    .with_title(format!(
                        "Settings - Bean Cypher v{}-dev",
                        env!("CARGO_PKG_VERSION")
                    ))
                    .with_maximize_button(false)
                    .with_inner_size([400.0, 200.0]),
                |ctx, _class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Settings");
                        ui.label("(The settings don't get saved. FOR NOW!)");

                        ui.separator();

                        egui::global_dark_light_mode_buttons(ui);

                        egui::Grid::new("settings_grid").show(ui, |ui| {
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
    }
}
