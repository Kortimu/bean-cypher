use std::{fs, io};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/bean.png")[..])
                    .expect("Failed loading the icon.")
            ),
        ..Default::default()
    };
    eframe::run_native(
        &format!("Bean Cypher Alpha v{}-dev", env!("CARGO_PKG_VERSION")),
        native_options,
        Box::new(|cc| Ok(Box::new(bean_cypher::BeanCypherApp::new(cc))))
    )

    // main_menu();
}

#[allow(clippy::missing_panics_doc)]
pub fn main_menu() {
    let version = env!("CARGO_PKG_VERSION");

    println!();
    println!("=======================================");
    println!("Bean Cypher Alpha v{version} - Coming Soon™");
    println!("=======================================");
    println!("(e)ncode text");
    println!("(d)ecode text");
    println!("(q)uit");
    println!("---------------------------------------");
    println!("append \"f\" to read from text file (ef/df)");
    println!("=======================================");

    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("fellas we fucked up");

    match command.trim().to_lowercase().as_str() {
        "e" => bean_cypher::encode_prep::run(),
        "ef" => bean_cypher::encode_prep::run_file(),
        "d" => bean_cypher::decode_prep::run(),
        "df" => bean_cypher::decode_prep::run_file(),
        "q" => std::process::exit(0),
        _ => {
            bean_cypher::notifications::error_message("Wrong command. Just type in a single letter in the command prompt.");
            main_menu();
        }
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn again(output: String) {
    println!();
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("(r)epeat command");
    println!("(w)rite output to text file");
    println!("type anything else to quit");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("really dude");

    match command.trim().to_lowercase().as_str() {
        "r" => main_menu(),
        "w" => {
            // TODO: see those test cases maybe idk?
            let _ = fs::write("output.txt", output);

            bean_cypher::notifications::info_message("file written successfully i hope :3");
            main_menu();
        },
        _ => std::process::exit(0)
    }
}