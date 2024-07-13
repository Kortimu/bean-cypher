use std::result::Result::Ok;

use crate::{hash_convert::hash_conversions::id_to_string, ErrorState};
use std::collections::HashMap;

// TODO: learn how to document some of this
// Ok() returns the output AND warning message (if there are none, an empty string)
pub fn run(text: &str) -> Result<(String, String), ErrorState> {
    let trimmed_text = text.trim();
    let mut beans = trimmed_text.split(' ');

    let text_major: usize;
    let text_minor: usize;

    if let Some(result) = beans.next() {
        text_major = decode_beans(result);
    } else {
        // return Err("Decoding error: Input lacks info.".to_string())
        return Err(ErrorState::Error(
            "Decoding error: Input lacks info.".to_string(),
        ));
    }
    if let Some(result) = beans.next() {
        text_minor = decode_beans(result);
    } else {
        // return Err("Decoding error: Input lacks info.".to_string())
        return Err(ErrorState::Error(
            "Decoding error: Input lacks info.".to_string(),
        ));
    }

    // compares this program's version to the version the text was encoded in
    let warning_msg = check_version(text_major, text_minor)?;

    let mut output = String::new();

    for bean in beans {
        let id = decode_beans(bean);

        // convert each id to what it means
        let string = id_to_string(id);
        output.insert_str(output.len(), &string);
    }

    Ok((output, warning_msg))
}

fn check_version(text_major: usize, text_minor: usize) -> Result<String, ErrorState> {
    let program_major: usize;
    let program_minor: usize;

    match env!("CARGO_PKG_VERSION_MAJOR").parse() {
        Ok(result) => program_major = result,
        Err(error) => {
            return Err(ErrorState::Error(format!(
                "{}: Flawed version major value in Cargo.toml.",
                error
            )))
        }
    }
    match env!("CARGO_PKG_VERSION_MINOR").parse() {
        Ok(result) => program_minor = result,
        Err(error) => {
            return Err(ErrorState::Error(format!(
                "{}: Flawed version major value in Cargo.toml.",
                error
            )))
        }
    }

    if program_major != text_major || program_minor != text_minor {
        return Ok(format!("Warning: the text might get decoded wrong due to mismatched versions.\nEncoded in v{text_major}.{text_minor}.x\nDecoded in v{program_major}.{program_minor}.x"));
    }

    Ok(String::new())
}

fn decode_beans(bean: &str) -> usize {
    // TODO: check from 1 whole hash, iterate yada yada
    let b_hash = HashMap::from([("b", 0), ("B", 72), ("6", 144)]);
    let e_hash = HashMap::from([("e", 0), ("E", 24), ("3", 48)]);
    let a_hash = HashMap::from([("a", 0), ("A", 8), ("4", 16)]);
    let n_hash = HashMap::from([("n", 0), ("N", 4)]);
    let s_hash = HashMap::from([("s", 0), ("S", 1), ("5", 2), ("", 3)]);

    let mut id = 0;

    let mut chars = bean.chars();
    let mut current_char;

    // yoink first character, convert to string, check against hash and add to id. do this 5 times. is there a better way? probably but i am learning so any sins and felonies can be forgiven
    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = String::new(),
    }
    if let Some(result) = b_hash.get(&current_char.as_str()) {
        id += result;
    }

    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = String::new(),
    }
    if let Some(result) = e_hash.get(&current_char.as_str()) {
        id += result;
    }

    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = String::new(),
    }
    if let Some(result) = a_hash.get(&current_char.as_str()) {
        id += result;
    }

    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = String::new(),
    }
    if let Some(result) = n_hash.get(&current_char.as_str()) {
        id += result;
    }

    match chars.next() {
        Some(result) => current_char = result.to_string(),
        None => current_char = String::new(),
    }
    // special case for none because i am figuring out how to make a good system later. it works™
    match s_hash.get(&current_char.as_str()) {
        Some(result) => id += result,
        None => id += 3,
    }

    id
}
