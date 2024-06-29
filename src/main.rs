use std::{fs, io};
mod hash_convert;
#[path = "encode.rs"] mod encode;
#[path = "encode_prep.rs"] mod encode_prep;
#[path = "decode.rs"] mod decode;
#[path = "decode_prep.rs"] mod decode_prep;

fn main() {
    let version = env!("CARGO_PKG_VERSION");

    println!("");
    println!("=======================================");
    println!("Bean Cypher Alpha v{} - Coming Soon™", version);
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
            println!("");
            println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            println!("Wrong command. Just type in a single letter in the command prompt.");
            println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            main();
        }
    }
}

pub fn again(output: Option<String>) {
    println!("");
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
            let _ = fs::write("output.txt", output.unwrap_or(String::from("There was a problem with writing to the file. whoops")));

            println!("---------------------------------------");
            println!("file written successfully i hope :3");
            println!("---------------------------------------");

            main();
        },
        _ => std::process::exit(0)
    }
}