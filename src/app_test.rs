use crate::ErrorType;
use crate::ManualSection;
use crate::Tab;

use crate::app_test::credits::show_credits;
use crate::app_test::cypher::show_main_menu;
use crate::app_test::manual::show_manual;
use crate::app_test::manual::show_manual_sidebar;
use crate::app_test::settings::show_settings;

#[path = "tabs/credits.rs"]
mod credits;
#[path = "tabs/cypher.rs"]
mod cypher;
#[path = "tabs/manual.rs"]
mod manual;
#[path = "tabs/settings.rs"]
mod settings;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TestApp {
    input: String,
    #[serde(skip)]
    output: String,
    #[serde(skip)]
    current_tab: Tab,
    manual_section: ManualSection,

    set_lowoutput: bool,

    #[serde(skip)]
    // TODO: might wanna merge with active_error -> active_result
    output_shown: bool,
    #[serde(skip)]
    // TODO: might want result? i dunno
    active_error: Option<ErrorType>,
    // TODO: later add a patch version also, for shits and giggles
    pub newest_version_found: Option<(u32, u32)>
}

impl Default for TestApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            manual_section: ManualSection::Intro,
            current_tab: Tab::Cypher,

            set_lowoutput: false,

            output_shown: false,
            active_error: None,
            newest_version_found: None
        }
    }
}

impl TestApp {
    /// Called once before the first frame.
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc.storage.map_or_else(Self::default, |storage| {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        })
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
                    for tab in [Tab::Cypher, Tab::Settings, Tab::Manual, Tab::Credits]
                        .into_iter()
                        .enumerate()
                    {
                        let picked = self.current_tab == tab.1;

                        let dat_button = egui::Button::selectable(
                            picked,
                            egui::RichText::new(format!("{0:?}", tab.1).to_lowercase()).size(20.0),
                        );

                        if cols[tab.0]
                            .add_sized([cols[tab.0].available_width(), 40.0], dat_button)
                            .clicked()
                        {
                            self.current_tab = tab.1;
                        }
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
            show_manual_sidebar(ui, self);
        }

        egui::CentralPanel::default().show_inside(ui, |ui| match self.current_tab {
            Tab::Cypher => {
                show_main_menu(ui, self);
            }
            Tab::Settings => {
                show_settings(ui, self);
            }
            Tab::Manual => {
                show_manual(ui);
            }
            Tab::Credits => {
                show_credits(ui);
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
        // TODO: random message potential ngl
        ui.label(". also hi :D");
    });
}
