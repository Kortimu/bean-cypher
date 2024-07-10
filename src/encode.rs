use crate::hash_convert::hash_conversions::{find_phrases, id_to_string, string_to_id};
use std::collections::HashMap;

pub fn run(text: &str) -> String {
    let mut ids = Vec::new();

    let phrases = find_phrases(text);

    // convert each character to an id
    for char in text.chars() {
        let id = string_to_id(&char.to_string());
        ids.insert(ids.len(), id);
    }

    // blast in the phrases in ids (god this is gonna be fun won't it)
    for phrase in phrases {
        let index = phrase.0;
        let id = phrase.1;
        let length = id_to_string(id).chars().count();

        // -1 is already filtered, gotta avoid filtering twice my god that would suck
        let mut taken = false;
        for i in index..index + length - 1 {
            if *ids
                .get(i)
                .expect("Phrase ID does not exist where it should. Somehow. what")
                == usize::MAX
            {
                taken = true;
            }
        }

        if !taken {
            ids.remove(index);
            ids.insert(index, id);

            // replaces every character in the phrase with -1 besides the first character
            for i in 1..length {
                let _ = std::mem::replace(&mut ids[index + i], usize::MAX);
            }
        }
    }

    // print first two "beans" to mark the major and minor version of the program this was encoded in
    // this only breaks when either major or minor is > 215. surely this shitpost won't go that far?
    let version_major = env!("CARGO_PKG_VERSION_MAJOR")
        .parse::<usize>()
        .expect("Error parsing package's major version in Cargo.toml.");
    let version_minor = env!("CARGO_PKG_VERSION_MINOR")
        .parse::<usize>()
        .expect("Error parsing package's minor version in Cargo.toml.");

    ids.insert(0, version_major);
    ids.insert(1, version_minor);

    let mut output = String::new();

    // convert each id to its corresponding "beans"
    // TODO: each can have its hash, sure, but how bout making the match not repeat 5 times?
    for id in ids {
        // removal of dummies
        if id != usize::MAX {
            let b_part = id / 72;
            let b_hash = HashMap::from([(0, "b"), (1, "B"), (2, "6")]);
            if let Some(result) = b_hash.get(&b_part) {
                output.insert_str(output.len(), result);
            }

            let e_part = (id % 72) / 24;
            let e_hash = HashMap::from([(0, "e"), (1, "E"), (2, "3")]);
            if let Some(result) = e_hash.get(&e_part) {
                output.insert_str(output.len(), result);
            }

            let a_part = (id % 24) / 8;
            let a_hash = HashMap::from([(0, "a"), (1, "A"), (2, "4")]);
            if let Some(result) = a_hash.get(&a_part) {
                output.insert_str(output.len(), result);
            }

            let n_part = (id % 8) / 4;
            let n_hash = HashMap::from([(0, "n"), (1, "N")]);
            if let Some(result) = n_hash.get(&n_part) {
                output.insert_str(output.len(), result);
            }

            let s_part = id % 4;
            let s_hash = HashMap::from([(0, "s"), (1, "S"), (2, "5"), (3, "")]);
            if let Some(result) = s_hash.get(&s_part) {
                output.insert_str(output.len(), result);
            }

            // beans beans >>> beansbeans
            output.insert(output.len(), ' ');
        }
    }
    // remove final space after final bean
    output.pop();
    output
}
