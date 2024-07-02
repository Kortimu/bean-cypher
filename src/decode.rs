use crate::{again, hash_convert::hash_convert::id_to_string, main, notifications::error_message};
use std::collections::HashMap;
use configparser::ini::Ini;

pub fn run(text: &str) {
    let mut beans = text.split(' ');

    let mut text_major = 69;
    let mut text_minor = 69;

    if let Some(result) = beans.next() {
        text_major = decode_beans(result);
    } else {
        error_message("The given text could not be decoded. Are you sure it's the right one mate?");
        main();
    }
    if let Some(result) = beans.next() {
        text_minor = decode_beans(result);
    } else {
        error_message("The given text could not be decoded. Are you sure it's the right one mate?");
        main();
    }

    // compares this program's version to the version the text was encoded in
    check_version(text_major, text_minor);

    let mut output = String::new();

    for bean in beans {
        let id = decode_beans(bean);

        // convert each id to what it means
        let string = id_to_string(id);
        output.insert_str(output.len(), &string);
    }

    let mut config = Ini::new();
    let lowercase_output = if config.load("./config.ini").is_ok() {
        config.getbool("settings", "lowercase_output").unwrap_or(Some(false)).unwrap_or(false)
    } else {
        false
    };

    if lowercase_output {
        output = output.to_lowercase();
    }

    print!("{output}");
    again(output);
}

fn check_version(text_major: usize, text_minor: usize) {
    let program_major = env!("CARGO_PKG_VERSION_MAJOR").parse::<usize>().expect("Error parsing package's major version in Cargo.toml.");
    let program_minor = env!("CARGO_PKG_VERSION_MINOR").parse::<usize>().expect("Error parsing package's minor version in Cargo.toml.");

    if program_major != text_major || program_minor != text_minor {
        println!();
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        println!("The text was encoded in a different version of the program. It may get decoded wrongly.");
        println!("- - - - - - - - - - - - - - - - - - - -");
        println!("Text was encoded in v{text_major}.{text_minor}.x");
        println!("Text is decoded in v{program_major}.{program_minor}.x");
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        println!();
    }
}

fn decode_beans (bean: &str) -> usize {
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
        None => current_char = String::new()
    }
    if let Some(result) = b_hash.get(&current_char.as_str()) {
        id += result;
    }

    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = String::new()
    }
    if let Some(result) = e_hash.get(&current_char.as_str()) {
        id += result;
    }

    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = String::new()
    }
    if let Some(result) = a_hash.get(&current_char.as_str()) {
        id += result;
    }

    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = String::new()
    }
    if let Some(result) = n_hash.get(&current_char.as_str()) {
        id += result;
    }

    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = String::new()
    }
    // special case for none because i am figuring out how to make a good system later. it works™
    match s_hash.get(&current_char.as_str()) {
        Some(result) => id += result,
        None => id += 3
    }

    id
}