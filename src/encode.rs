use crate::{again, hash_convert::hash_convert::{string_to_id, find_phrases, id_to_string}};
use std::{collections::HashMap, io};

pub fn run() {
    println!("---------------------------------------");
    println!("we encodin baby! type yer sentence :3");
    println!("---------------------------------------");

    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("god damn it");

    let mut ids = Vec::new();

    let phrases = find_phrases(&text);

    // convert each character to an id
    for char in text.chars() {
        let id = string_to_id(char.to_string());
        ids.insert(ids.len(), id);
    }
    
    // blast in the phrases in ids (god this is gonna be fun won't it)
    for phrase in phrases {
        let index = phrase.0;
        let id = phrase.1;
        let length = id_to_string(id).chars().count();

        // -1 is already filtered, gotta avoid filtering twice my god that would suck
        let mut taken = false;
        for i in index..index + length as i32 - 1 {
            if ids.get(i as usize).unwrap().to_owned() == -1 {
                taken = true;
            }
        }

        // if ids.get(index as usize).unwrap().to_owned() != -1 {
        if taken == false {
            ids.remove(index.try_into().unwrap());
            ids.insert(index.try_into().unwrap(), id);
    
            // replaces every character in the phrase with -1 besides the first character
            for i in 1..length {
                let _ = std::mem::replace(&mut ids[index as usize + i], -1);
            }
        }
    }

    // print first two "beans" to mark the major and minor version of the program this was encoded in
    // this only breaks when either major or minor is > 215. surely this shitpost won't go that far?
    let version_major = env!("CARGO_PKG_VERSION_MAJOR").parse::<i32>().unwrap();
    let version_minor = env!("CARGO_PKG_VERSION_MINOR").parse::<i32>().unwrap();

    ids.insert(0, version_major);
    ids.insert(1, version_minor);

    println!("---------------------------------------");

    let mut output = String::new();

    // convert each id to its corresponding "beans"
    // TODO: each can have its hash, sure, but how bout making the match not repeat 5 times?
    for id in ids {
        // removal of dummies
        if id != -1 {
            let b = id / 72;
            let b_hash = HashMap::from([
                (0, "b"),
                (1, "B"),
                (2, "6")
            ]);
            match b_hash.get(&b) {
                Some(result) => output.insert_str(output.len(), result),
                None => ()
            };
    
            let e = (id % 72) / 24;
            let e_hash = HashMap::from([
                (0, "e"),
                (1, "E"),
                (2, "3")
            ]);
            match e_hash.get(&e) {
                Some(result) => output.insert_str(output.len(), result),
                None => ()
            }
    
            let a = (id % 24) / 8;
            let a_hash = HashMap::from([
                (0, "a"),
                (1, "A"),
                (2, "4")
            ]);
            match a_hash.get(&a) {
                Some(result) => output.insert_str(output.len(), result),
                None => ()
            }
    
            let n = (id % 8) / 4;
            let n_hash = HashMap::from([
                (0, "n"),
                (1, "N")
            ]);
            match n_hash.get(&n) {
                Some(result) => output.insert_str(output.len(), result),
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
                Some(result) => output.insert_str(output.len(), result),
                None => ()
            }
    
            // beans beans >>> beansbeans
            output.insert_str(output.len(), " ");
        }
    }
    print!("{}", output);
    again(Some(output));
}