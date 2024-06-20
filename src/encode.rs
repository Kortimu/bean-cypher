use crate::{again, hash_convert::hash_convert::char_to_id};
use std::{collections::HashMap, io};

pub fn run() {
    println!("---------------------------------------");
    println!("we encodin baby! type yer sentence :3");
    println!("---------------------------------------");

    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("god damn it");

    let mut ids = Vec::new();

    // convert each character to an id
    for char in text.chars() {
        ids.append(&mut char_to_id(char));
    }

    // print first two "beans" to mark the major and minor version of the program this was encoded in
    // this only breaks when either major or minor is > 215. surely this shitpost won't go that far?
    let version_major = env!("CARGO_PKG_VERSION_MAJOR").parse::<i32>().unwrap();
    let version_minor = env!("CARGO_PKG_VERSION_MINOR").parse::<i32>().unwrap();

    ids.insert(0, version_major);
    ids.insert(1, version_minor);

    println!("---------------------------------------");

    // convert each id to its corresponding "beans"
    // TODO: each can have its hash, sure, but how bout making the match not repeat 5 times?
    for id in ids {
        let b = id / 72;
        let b_hash = HashMap::from([
            (0, "b"),
            (1, "B"),
            (2, "6")
        ]);
        match b_hash.get(&b) {
            Some(result) => print!("{}", result),
            None => ()
        }

        let e = (id % 72) / 24;
        let e_hash = HashMap::from([
            (0, "e"),
            (1, "E"),
            (2, "3")
        ]);
        match e_hash.get(&e) {
            Some(result) => print!("{}", result),
            None => ()
        }

        let a = (id % 24) / 8;
        let a_hash = HashMap::from([
            (0, "a"),
            (1, "A"),
            (2, "4")
        ]);
        match a_hash.get(&a) {
            Some(result) => print!("{}", result),
            None => ()
        }

        let n = (id % 8) / 4;
        let n_hash = HashMap::from([
            (0, "n"),
            (1, "N")
        ]);
        match n_hash.get(&n) {
            Some(result) => print!("{}", result),
            None => ()
        }

        let s = id % 4;
        let s_hash = HashMap::from([
            (0, "s"),
            (1, "S"),
            (2, "5"),
            (3, "")
        ]);
        match s_hash.get(&s) {
            Some(result) => print!("{}", result),
            None => ()
        }

        // beans beans >>> beansbeans
        print!(" ");
    }

    again();
}