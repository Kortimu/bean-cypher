#[path = "encode.rs"] mod encode;
// #[path = "decode.rs"] mod decode;
use crate::decode;

#[derive(Default)]
pub struct BeanCypherApp {
    input: String,
    output: String,
    show_settings: bool
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
        Self::default()
    }
}

impl eframe::App for BeanCypherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.columns(2, |columns| {
                    // columns[0].image("../assets/bean.png");
                    columns[0].heading("Bean Cypher");
                    columns[0].label(format!("Alpha v{}-dev", env!("CARGO_PKG_VERSION")));
                    if columns[1].button("Open Settings").clicked() {
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
                    .with_title(format!("Settings - Bean Cypher v{}-dev", env!("CARGO_PKG_VERSION"))),
                |ctx, _class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Coming Soon™")
                    })
                }
            );
        }
    }
}