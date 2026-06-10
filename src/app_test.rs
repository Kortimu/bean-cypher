use crate::ManualSection;
use crate::Tab;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TestApp {
    label: String,
    current_tab: Tab,
    manual_section: ManualSection,

    #[serde(skip)]
    value: f32,
}

impl Default for TestApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            manual_section: ManualSection::Intro,
            current_tab: Tab::Cypher,
        }
    }
}

impl TestApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TestApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.columns(4, |cols| {
                    let mut i = 0;
                    for (tab, name) in [
                        (Tab::Cypher, "cypher"),
                        (Tab::Settings, "settings"),
                        (Tab::Manual, "manual"),
                        (Tab::Credits, "credits"),
                    ] {
                        let picked = self.current_tab == tab;

                        let dat_button =
                            egui::Button::selectable(picked, egui::RichText::new(name).size(20.0));

                        if cols[i]
                            .add_sized([cols[i].available_width(), 40.0], dat_button)
                            .clicked()
                        {
                            self.current_tab = tab;
                        }

                        i += 1;
                    }
                });
            });
        });

        egui::Panel::bottom("extra_info").show_inside(ui, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        if self.current_tab == Tab::Manual {
            egui::Panel::left("manual_headings").show_inside(ui, |ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(200.0, ui.available_height()),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            egui::Frame::group(ui.style())
                                .outer_margin(5.0)
                                .show(ui, |ui| {
                                    ui.set_min_height(ui.available_height());
                                    for section in [
                                        ManualSection::Intro,
                                        ManualSection::Vision,
                                        ManualSection::Encrypting,
                                        ManualSection::Decrypting,
                                        ManualSection::Settings,
                                        ManualSection::FileShenanigans,
                                        ManualSection::Roadmap,
                                        ManualSection::Faq,
                                        ManualSection::Thanks,
                                        ManualSection::Social,
                                    ] {
                                        ui.selectable_value(
                                            &mut self.manual_section,
                                            section,
                                            format!("{section:?}").to_lowercase(),
                                        );
                                    }
                                });
                        });
                    },
                );
            });
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            match self.current_tab {
                Tab::Cypher => {
                    ui.heading("CYPHER!!!!");

                    ui.horizontal(|ui| {
                        ui.label("Write something: ");
                        ui.text_edit_singleline(&mut self.label);
                    });

                    ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
                    if ui.button("Increment").clicked() {
                        self.value += 1.0;
                    }

                    ui.separator();

                    ui.add(egui::github_link_file!(
                        "https://github.com/emilk/eframe_template/blob/main/",
                        "Source code."
                    ));

                    if self.value == 10.0 {
                        egui::Modal::new(egui::Id::new("whoopsie_daisie"))
                            .show(ui, |ui| {
                                ui.label("mama mia");
                                if ui.button("OK").clicked() {
                                    self.value = 0.0;
                                }
                            });
                    }
                },
                Tab::Settings => {
                    ui.heading("SETTINGS!!!!");
                },
                Tab::Manual => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Frame::group(ui.style()).show(ui, |ui| {
                            ui.allocate_ui_with_layout(
                                ui.available_size(),
                                egui::Layout::top_down(egui::Align::LEFT),
                                |ui| {
                                    ui.label("sigh starch lord is the most overrated card in the entire game. I can't stand it when people think starch lord is good. You guys, this is a 4 cost card that has 2 attack and 4 health. It has garbage stats. Now let's look at it's abilities. When you play a root, it gets +1/+1. Thats almost no- think about it. In order to get this up to the actual stats that it needs to be as a 4 cost card, this would have to buff like 2 roots just to break even. It would have to grow 3 roots to actually be viable. The fact that people think that it is a good idea to start drawing cards on turn 5, it really means you have no idea how to play pvz heroes. It's way too late! This is- it's- you dont make a deck that has roots in it, that's not a good strategy, there are some good roots in the game but you just have to put too many roots in it. It's drawing cards on turn 5, the last turn you're gonna be drawing cards as a plant player is going to be on turn 3. it's so overrated, it's just a big piece of trash, just look at this guy, a big ugly guy. it's based on by the way the worst Marvel superhero in the entire Marvel Fra- actually universe of Superheroes in the entire history of the planet, star-lord, who is a simp, douchebag, has no superpowers, is the lamest, dumb. and do you know what, it's appropriate cuz this is the stupidest card in the game and it's based on the stupidest Marvel superhero ever. This is so overrated, It's so grunts it's so, I'm sticking this in F tier I don't even care.");
                                },
                            );
                        });
                    });
                },
                Tab::Credits => {
                    ui.heading("credits or whatever");
                }
            }
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
