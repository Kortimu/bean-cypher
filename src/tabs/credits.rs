pub fn show_credits(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.add(
            egui::Image::new(
                egui::include_image!("../../assets/beano.png")
            ).fit_to_exact_size(egui::vec2(105.0, 105.0))
        );
        ui.vertical(|ui| {
            ui.heading(format!("bean cypher alpha v{}", env!("CARGO_PKG_VERSION")));
            ui.horizontal(|ui| {
                ui.label("evil mastermind behind this:");
                ui.hyperlink_to("kortimu :]", "https://kortimu.github.io");
            });
            ui.horizontal(|ui| {
                ui.label("chief bean officer (cbo):");
                ui.hyperlink_to("bean man", "https://twitch.tv/beandhd");
            });
            ui.horizontal(|ui| {
                ui.label("enabler of this:");
                ui.hyperlink_to("acer extensa 215-55", "https://www.google.com/search?q=some+laptop+i+found+or+something+i+dunno&tbm=isch");
            });
            ui.horizontal(|ui| {
                ui.hyperlink_to(" source code", "https://github.com/Kortimu/bean-cypher");
                ui.hyperlink_to("🎶 gensokyo radio", "https://gensokyoradio.net");
            });
        })
    });

    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
        ui.add(
            egui::Image::new(
                egui::include_image!("../../assets/funny_image.png")
            ).fit_to_exact_size(egui::vec2(300.0, 165.0))
        );
        ui.vertical(|ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                ui.label("additional thanks to:");
                ui.horizontal(|ui| {
                    ui.label("for being awesome");
                    ui.hyperlink_to("n0o0b090lv", "https://youtube.com");
                });
                ui.horizontal(|ui| {
                    ui.label("for being awesome");
                    ui.hyperlink_to("Makazis", "https://youtube.com");
                });
                ui.label("and to any translators i pick up");
                ui.label("Enter Text Here");
                ui.label("Sample Text");
                ui.label("Even Cooler Sample Text");
                ui.label("The Coolest Sample Text You Have Seen");
                ui.label("my friend uses arch btw");
            });
        });
    });

}
