use std::{fs::File, io::{self, Read}};
use crate::{encode, encode_prep, notifications::{error_message, info_message}};

pub fn run() {
    info_message("we decodin baby! type yer nonsense :3");

    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("god damn it");

    encode::run(&text);
}

pub fn run_file() {
    info_message("we decodin baby! type yer text file :3");

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("god damn it");
    file_name.pop();
    file_name.pop();

    let file_path = "./".to_owned() + &file_name + ".txt";
    let potential_file = File::open(file_path);

    if potential_file.is_ok() {
        let mut file = potential_file.expect("ah fuck");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("ah fuck");
        encode::run(&contents);
    } else {
        error_message("Wrong file name or file doesn't exist next to executable.");
        encode_prep::run_file();
    }
}