use std::{fs::File, io::{self, Read}};
use crate::{decode, decode_prep};

pub fn run() {
    println!("---------------------------------------");
    println!("we decodin baby! type yer nonsense :3");
    println!("---------------------------------------");

    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("god damn it");

    println!("---------------------------------------");

    decode::run(text);
}

pub fn run_file() {
    println!("---------------------------------------");
    println!("we decodin baby! type yer text file :3");
    println!("---------------------------------------");

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("god damn it");
    file_name.pop();
    file_name.pop();
    
    let file_path = "./".to_owned() + &file_name + ".txt";
    let potential_file = File::open(file_path);

    // TODO: error if can't find file

    if potential_file.is_ok() {
        let mut file = potential_file.expect("ah fuck");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("ah fuck");
        decode::run(contents);
    } else {
        println!("");
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        println!("Wrong file name or file doesn't exist next to executable.");
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        decode_prep::run_file();
    }
}