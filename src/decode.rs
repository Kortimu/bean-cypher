use crate::{again, utils::hash_convert::print_from_id};
use std::{collections::HashMap, io};

pub fn run() {
    println!("---------------------------------------");
    println!("we decodin baby! type yer nonsense :3");
    println!("---------------------------------------");

    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("god damn it");

    println!("---------------------------------------");
    
    let mut beans = text.split(' ');

    let text_major = decode_beans(beans.next().unwrap());
    let text_minor = decode_beans(beans.next().unwrap());

    // compares this program's version to the version the text was encoded in
    check_version(text_major, text_minor);

    for bean in beans {
        let id = decode_beans(bean);

        // convert each id to what it means
        print_from_id(id);
    }   

    again();
}

fn check_version(text_major: i32, text_minor: i32) {
    let program_major = env!("CARGO_PKG_VERSION_MAJOR").parse::<i32>().unwrap();
    let program_minor = env!("CARGO_PKG_VERSION_MINOR").parse::<i32>().unwrap();

    if program_major != text_major || program_minor != text_minor {
        println!("");
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        println!("The text was encoded in a different version of the program. It may get decoded wrongly.");
        println!("- - - - - - - - - - - - - - - - - - - -");
        println!("Text was encoded in v{}.{}.x", text_major, text_minor);
        println!("Text is decoded in v{}.{}.x", program_major, program_minor);
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        println!("");
    }
}

fn decode_beans (bean: &str) -> i32 {
    let b_hash = HashMap::from([
        ("b", 0),
        ("B", 72),
        ("6", 144),
    ]);
    let e_hash = HashMap::from([
        ("e", 0),
        ("E", 24),
        ("3", 48),
    ]);
    let a_hash = HashMap::from([
        ("a", 0),
        ("A", 8),
        ("4", 16),
    ]);
    let n_hash = HashMap::from([
        ("n", 0),
        ("N", 4),
    ]);
    let s_hash = HashMap::from([
        ("s", 0),
        ("S", 1),
        ("5", 2),
        ("", 3),
    ]);

    let mut id = 0;

    let mut chars = bean.chars();
    let mut current_char;

    // yoink first character, convert to string, check against hash and add to id. do this 5 times. is there a better way? probably but i am learning so any sins and felonies can be forgiven
    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = "".to_string()
    }
    match b_hash.get(&current_char.as_str()) {
        Some(result) => id = id + result,
        None => ()
    }

    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = "".to_string()
    }
    match e_hash.get(&current_char.as_str()) {
        Some(result) => id = id + result,
        None => ()
    }

    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = "".to_string()
    }
    match a_hash.get(&current_char.as_str()) {
        Some(result) => id = id + result,
        None => ()
    }

    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = "".to_string()
    }
    match n_hash.get(&current_char.as_str()) {
        Some(result) => id = id + result,
        None => ()
    }

    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = "".to_string()
    }
    // special case for none because i am figuring out how to make a good system later. it works™
    match s_hash.get(&current_char.as_str()) {
        Some(result) => id = id + result,
        None => id = id + 3
    }

    return id;
}