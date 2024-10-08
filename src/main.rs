#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/bean.png")[..])
                    .expect("Failed loading the icon."),
            )
            .with_title(format!(
                "Bean Cypher (Alpha v{}-dev)",
                env!("CARGO_PKG_VERSION")
            )),
        persist_window: false,
        ..Default::default()
    };
    eframe::run_native(
        "Bean Cypher",
        native_options,
        Box::new(|cc| Ok(Box::new(bean_cypher::BeanCypher::new(cc)))),
    )
}
