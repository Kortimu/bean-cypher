use std::io;
mod hash_convert;
#[path = "encode.rs"] mod encode;
#[path = "decode.rs"] mod decode;
#[macro_use]
extern crate lazy_static;

fn main() {
    let version = env!("CARGO_PKG_VERSION");

    println!("");
    println!("=======================================");
    println!("Bean Cypher Alpha v{} - Coming Soon™", version);
    println!("=======================================");
    println!("type e => encode sentence");
    println!("type d => decode sentence");
    println!("type q => quit");
    println!("=======================================");

    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("fellas we fucked up");

    match command.trim().to_lowercase().as_str() {
        "e" => encode::run(),
        "d" => decode::run(),
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

pub fn again() {
    println!("");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("type r - try again");
    println!("type anything else - quit");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("really dude");

    match command.trim().to_lowercase().as_str() {
        "r" => main(),
        _ => std::process::exit(0)
    }
}