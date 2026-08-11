use crate::ErrorType;
use crate::Language;
use crate::ManualSection;
use crate::Tab;
use crate::ENGLISH;
use egui_i18n::tr;

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
    // TODO: one day figure out how to remove this
    #[serde(skip)]
    manual_lang: Language,

    set_lowoutput: bool,
    set_lang: String,
    set_theme: egui::ThemePreference,

    #[serde(skip)]
    // TODO: might wanna merge with active_error -> active_result
    output_shown: bool,
    #[serde(skip)]
    // TODO: might want result? i dunno
    active_error: Option<ErrorType>,
    // TODO: later add a patch version also, for shits and giggles
    pub newest_version_found: Option<(u32, u32)>,
    // TODO: when rewriting decode.rs, make into a (u32, u32)
    output_warning: String,
}

impl Default for TestApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            current_tab: Tab::Cypher,
            manual_section: ManualSection::Intro,
            manual_lang: ENGLISH,

            set_lowoutput: false,
            set_lang: egui_i18n::get_language(),
            set_theme: egui::ThemePreference::System,

            output_shown: false,
            active_error: None,
            newest_version_found: None,
            output_warning: String::new(),
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

        cc.egui_ctx
            .include_bytes("bytes://funi", include_bytes!("../assets/funny_image.png"));
        cc.egui_ctx.include_bytes(
            "bytes://tutorial",
            include_bytes!("../assets/cypher_tutorial.png"),
        );
        cc.egui_ctx.include_bytes(
            "bytes://funi_stret",
            include_bytes!("../assets/street_beans.jpg"),
        );
        cc.egui_ctx.include_bytes(
            "bytes://funi_gun",
            include_bytes!("../assets/gun_beans.jpg"),
        );
        cc.egui_ctx.include_bytes(
            "bytes://funi_huh",
            include_bytes!("../assets/huh_beans.jpg"),
        );
        cc.egui_ctx.include_bytes(
            "bytes://funi_clen",
            include_bytes!("../assets/clean_beans.jpg"),
        );

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let app = cc.storage.map_or_else(Self::default, |storage| {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        });

        if let Err(err) = init_i18n(&app.set_lang) {
            eprintln!("error with i18n initialization: {err}");
        }

        app
    }
}

fn init_i18n(lang: &str) -> Result<(), Box<dyn std::error::Error>> {
    // On Windows, Fluent wraps placeables in Unicode directionality marks
    // (U+2068 / U+2069) that some native text renderers display as garbage.
    // Disable them before loading any bundles.
    #[cfg(target_os = "windows")]
    egui_i18n::set_use_isolating(false);

    egui_i18n::load_translations_from_path("assets/lang")?;

    egui_i18n::set_language(lang);
    egui_i18n::set_fallback("en-GB");
    Ok(())
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
                            egui::RichText::new(tr!(&format!("tab_{0:?}", tab.1).to_lowercase()))
                                .size(20.0),
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
                show_manual(ui, self);
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
