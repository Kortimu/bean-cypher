use crate::again;
use std::{collections::HashMap, io};

pub fn run() {
    println!("---------------------------------------");
    println!("we decodin baby! type yer nonsense :3");
    println!("---------------------------------------");

    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("god damn it");

    println!("---------------------------------------");

    let beans = text.split(' ');

    let char_hash = HashMap::from([
        (0, ' '),
        (1, 'A'),
        (2, 'Ā'),
        (3, 'B'),
        (4, 'C'),
        (5, 'Č'),
        (6, 'D'),
        (7, 'E'),
        (8, 'Ē'),
        (9, 'F'),
        (10, 'G'),
        (11, 'Ģ'),
        (12, 'H'),
        (13, 'I'),
        (14, 'Ī'),
        (15, 'J'),
        (16, 'K'),
        (17, 'Ķ'),
        (18, 'L'),
        (19, 'Ļ'),
        (20, 'M'),
        (21, 'N'),
        (22, 'Ņ'),
        (23, 'O'),
        (24, 'P'),
        (25, 'Q'),
        (26, 'R'),
        (27, 'S'),
        (28, 'Š'),
        (29, 'T'),
        (30, 'U'),
        (31, 'Ū'),
        (32, 'V'),
        (33, 'W'),
        (34, 'X'),
        (35, 'Y'),
        (36, 'Z'),
        (37, 'Ž')
    ]);

    for bean in beans {
        let id = decode_beans(bean);

        // convert each id to what it means
        match char_hash.get(&id) {
            Some(result) => print!("{}", result),
            None => ()
        }
    }   

    again();
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