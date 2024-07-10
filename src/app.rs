use egui::Rect;

use crate::encode;
use crate::decode;

#[derive(Default)]
pub struct BeanCypherApp {
    input: String,
    output: String,
    show_settings: bool,
    set_lowercase: bool
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
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.set_height(50.0);

                    ui.add(egui::Image::new(egui::include_image!("../assets/bean.png")).max_width(50.0));

                    ui.vertical(|ui| {
                        ui.add_space(5.0);
                        ui.heading("Bean Cypher");
                        ui.label(format!("Alpha v{}-dev", env!("CARGO_PKG_VERSION")));
                    });

                    let set_btn = ui.put(
                        Rect {
                            // there might be a less scuffed way of doing this but hey! it works
                            min: egui::Pos2 { x: ui.available_width() + 80.0, y: 10.0 },
                            max: egui::Pos2 { x: ui.available_width() + 170.0, y: 20.0 }
                        },
                        egui::Button::new("Open settings")
                    );
                    if set_btn.clicked() {
                        self.show_settings = true
                    }
                });

                ui.separator();
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized([ctx.input(|i| i.screen_rect().width() - 15.0), 100.0], egui::TextEdit::multiline(&mut self.input).hint_text("Input goes here...").clip_text(true));
                });

                ui.horizontal(|ui| {
                    if ui.button("Encode text").clicked() {
                        self.output = encode::run(&self.input);
                    }
                    if ui.button("Decode text").clicked() {
                        self.output = decode::run(&self.input);
                        if self.set_lowercase == true {
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
                    .with_title(format!("Settings - Bean Cypher v{}-dev", env!("CARGO_PKG_VERSION")))
                    .with_maximize_button(false),
                |ctx, _class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Settings");
                        ui.label("(The settings don't get saved. FOR NOW!)");

                        ui.separator();

                        egui::Grid::new("settings_grid").show(ui, |ui| {
                            ui.label("Lowercase output");
                                ui.checkbox(&mut self.set_lowercase, "")
                        });
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        self.show_settings = false;
                    }
                }
            );
        }
    }
}