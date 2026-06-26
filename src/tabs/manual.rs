use egui_commonmark::CommonMarkCache;
use egui_commonmark::CommonMarkViewer;

use crate::ManualSection;
use crate::TestApp;

pub fn show_manual_sidebar(ui: &mut egui::Ui, app: &mut TestApp) {
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
                                ManualSection::Basics,
                                ManualSection::Explanation,
                                ManualSection::CustomCyphers,
                                ManualSection::Edge,
                                ManualSection::Faq,
                                ManualSection::Thanks,
                                ManualSection::Sillies,
                            ] {
                                ui.style_mut().spacing.button_padding = egui::vec2(12.0, 8.0);
                                ui.selectable_value(
                                    &mut app.manual_section,
                                    section,
                                    section.label(),
                                );
                            }
                        });
                });
            },
        );
    });
}

pub fn show_manual(ui: &mut egui::Ui, app: &mut TestApp) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.allocate_ui_with_layout(
                ui.available_size(),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    let mut cache = CommonMarkCache::default();
                    CommonMarkViewer::new().show(ui, &mut cache, app.manual_section.file_path());

                    ui.separator();

                    // i need to find a home for this. staying here for now lol
                    ui.label("sigh starch lord is the most overrated card in the entire game. I can't stand it when people think starch lord is good. You guys, this is a 4 cost card that has 2 attack and 4 health. It has garbage stats. Now let's look at it's abilities. When you play a root, it gets +1/+1. Thats almost no- think about it. In order to get this up to the actual stats that it needs to be as a 4 cost card, this would have to buff like 2 roots just to break even. It would have to grow 3 roots to actually be viable. The fact that people think that it is a good idea to start drawing cards on turn 5, it really means you have no idea how to play pvz heroes. It's way too late! This is- it's- you dont make a deck that has roots in it, that's not a good strategy, there are some good roots in the game but you just have to put too many roots in it. It's drawing cards on turn 5, the last turn you're gonna be drawing cards as a plant player is going to be on turn 3. it's so overrated, it's just a big piece of trash, just look at this guy, a big ugly guy. it's based on by the way the worst Marvel superhero in the entire Marvel Fra- actually universe of Superheroes in the entire history of the planet, star-lord, who is a simp, douchebag, has no superpowers, is the lamest, dumb. and do you know what, it's appropriate cuz this is the stupidest card in the game and it's based on the stupidest Marvel superhero ever. This is so overrated, It's so grunts it's so, I'm sticking this in F tier I don't even care.");
                },
            );
        });
    });
}
