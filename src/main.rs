fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(
            eframe::icon_data::from_png_bytes(&include_bytes!("../assets/bean.png")[..])
                .expect("Failed loading the icon."),
        ),
        ..Default::default()
    };
    eframe::run_native(
        &format!("Bean Cypher Alpha v{}-dev", env!("CARGO_PKG_VERSION")),
        native_options,
        Box::new(|cc| Ok(Box::new(bean_cypher::BeanCypherApp::new(cc)))),
    )
}
