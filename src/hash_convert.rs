pub mod hash_convert {
    use configparser::ini::Ini;

    use crate::fs::File;
    use std::{  collections::HashMap, io::{BufRead, BufReader}};

    fn get_hash() -> HashMap<i32, String> {
        let string_hash: HashMap<i32, String> = HashMap::from([
            (0, "0".to_string()),
            (1, "1".to_string()),
            (2, "2".to_string()),
            (3, "3".to_string()),
            (4, "4".to_string()),
            (5, "5".to_string()),
            (6, "6".to_string()),
            (7, "7".to_string()),
            (8, "8".to_string()),
            (9, "9".to_string()),
            (10, " ".to_string()),
            (11, "A".to_string()),
            (12, "Ā".to_string()),
            (13, "B".to_string()),
            (14, "C".to_string()),
            (15, "Č".to_string()),
            (16, "D".to_string()),
            (17, "E".to_string()),
            (18, "Ē".to_string()),
            (19, "F".to_string()),
            (20, "G".to_string()),
            (21, "Ģ".to_string()),
            (22, "H".to_string()),
            (23, "I".to_string()),
            (24, "Ī".to_string()),
            (25, "J".to_string()),
            (26, "K".to_string()),
            (27, "Ķ".to_string()),
            (28, "L".to_string()),
            (29, "Ļ".to_string()),
            (30, "M".to_string()),
            (31, "N".to_string()),
            (32, "Ņ".to_string()),
            (33, "O".to_string()),
            (34, "P".to_string()),
            (35, "Q".to_string()),
            (36, "R".to_string()),
            (37, "S".to_string()),
            (38, "Š".to_string()),
            (39, "T".to_string()),
            (40, "U".to_string()),
            (41, "Ū".to_string()),
            (42, "V".to_string()),
            (43, "W".to_string()),
            (44, "X".to_string()),
            (45, "Y".to_string()),
            (46, "Z".to_string()),
            (47, "Ž".to_string()),
            (48, "!".to_string()),
            (49, "\'".to_string()),
            (50, "#".to_string()),
            (51, "$".to_string()),
            (52, "%".to_string()),
            (53, "&".to_string()),
            (54, "\"".to_string()),
            (55, "(".to_string()),
            (56, ")".to_string()),
            (57, "*".to_string()),
            (58, "+".to_string()),
            (59, ",".to_string()),
            (60, "-".to_string()),
            (61, ".".to_string()),
            (62, "/".to_string()),
            (63, ":".to_string()),
            (64, ";".to_string()),
            (65, "<".to_string()),
            (66, "=".to_string()),
            (67, ">".to_string()),
            (68, "?".to_string()),
            (69, "@".to_string()),
            (70, "[".to_string()),
            (71, "\\".to_string()),
            (72, "]".to_string()),
            (73, "^".to_string()),
            (74, "_".to_string()),
            (75, "`".to_string()),
            (76, "{".to_string()),
            (77, "|".to_string()),
            (78, "}".to_string()),
            (79, "~".to_string())
        ]);

        let mut correct_hash: HashMap<i32, String> = HashMap::new();

        let mut config = Ini::new();

        match config.load("./config.ini") {
            Ok(_) => {
                match config.get("settings", "cypher") {
                    Some(result) => {
                        let chosen_cypher_location = "./cyphers/".to_string() + &result + ".txt";

                        match File::open(chosen_cypher_location) {
                            Ok(cypher_file) => {
                                let buffered = BufReader::new(cypher_file);

                                for (hash_index, line) in buffered.lines().enumerate() {
                                    correct_hash.insert(hash_index as i32, line.unwrap());
                                }
                            }
                            Err(_) => {
                                correct_hash = string_hash;
                            }
                        }

                        
                    }
                    None => {
                        correct_hash = string_hash;
                    }
                }
            },
            Err(_) => {
                correct_hash = string_hash;
            }
        }
        correct_hash
    }

    pub fn id_to_string(id: i32) -> String {
        let string_hash = get_hash();
        
        if let Some(result) = string_hash.get(&id) {
            return result.to_string()
        }
        String::new()
    }

    // TODO: prioritize longer length strings
    // TODO: also can we try not nesting this much thanks
    pub fn find_phrases(text: &str) -> HashMap<usize, i32> {
        let upper_text = text.to_uppercase();
        let mut phrases: HashMap<usize, i32> = HashMap::new();
        let string_hash = get_hash();
        
        // afaik hashmaps pick randomly, we want to start with phrases that have the longest length to have as little bullshit as possible
        let mut string_hash_values = Vec::new();
        for value in string_hash.values() {
            string_hash_values.insert(string_hash_values.len(), value);
        }
        string_hash_values.sort_by_key(|k| k.len());
        string_hash_values.reverse();

        for string in string_hash_values {
            if string.chars().count() > 1 {
                let results: Vec<_> = upper_text.match_indices(string).collect();

                // tweaking the index to reference .chars().length instead of .len()
                // (please god forgive me for this jank)
                for result in results {
                    let mut previous_text = upper_text.clone();
                    previous_text.truncate(result.0);

                    let difference = previous_text.len() - previous_text.chars().count();
                    let correct_index = result.0 - difference;
                    phrases.insert(correct_index.max(0), string_to_id(result.1));
                }
            }
        }
        phrases
    }

    pub fn string_to_id(string: &str) -> i32 {
        let upper_char = &string.to_uppercase();
        let string_hash = get_hash();

        // iterate through values until we find a matching one, pass key (saves editing 2 different hashes)
        let potential_id = string_hash.iter().find_map(|(key, val)|
        if val == upper_char {
            Some(key)
        } else {
            None
        });

        // check for any unrecognized characters
        if potential_id.is_none() {
            return -1;
        }

        potential_id.unwrap().to_owned()
    }
}