use std::{fs, io};

use notifications::info_message;
mod hash_convert;
#[path = "notifications.rs"] mod notifications;
#[path = "encode_prep.rs"] mod encode_prep;
#[path = "decode_prep.rs"] mod decode_prep;
#[path = "encode.rs"] mod encode;
#[path = "decode.rs"] mod decode;

fn main() {
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
        "e" => encode_prep::run(),
        "ef" => encode_prep::run_file(),
        "d" => decode_prep::run(),
        "df" => decode_prep::run_file(),
        "q" => std::process::exit(0),
        _ => {
            notifications::error_message("Wrong command. Just type in a single letter in the command prompt.");
            main();
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
        "r" => main(),
        "w" => {
            // TODO: see those test cases maybe idk?
            let _ = fs::write("output.txt", output);

            info_message("file written successfully i hope :3");
            main();
        },
        _ => std::process::exit(0)
    }
}