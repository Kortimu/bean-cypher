pub mod hash_convert {
    use configparser::ini::Ini;

    use crate::fs::File;
    use std::{collections::HashMap, io::{BufRead, BufReader}};

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

                        let cypher_file = File::open(chosen_cypher_location);

                        let buffered = BufReader::new(cypher_file.unwrap());

                        // FIXME: i fucking hate this look how to make it not index fuck me
                        let mut quirky_index = 0;
                        for line in buffered.lines() {
                            correct_hash.insert(quirky_index, line.unwrap());
                            quirky_index += 1;
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
        return correct_hash;
    }

    pub fn id_to_string(id: i32) -> String {
        let string_hash = get_hash();
        match string_hash.get(&id) {
            Some(result) => return result.to_string(),
            None => ()
        }
        return "".to_string();
    }

    // TODO: prioritize longer length strings
    // TODO: also can we try not nesting this much thanks
    pub fn find_phrases(text: &str) -> HashMap<i32, i32> {
        let upper_text = text.to_uppercase();
        let mut phrases: HashMap<i32, i32> = HashMap::new();
        let string_hash = get_hash();

        for string in string_hash.values() {
            // those damn funny unicode characters!! (ā, ē, ū and the like)
            if string.chars().count() > 1 {
                match upper_text.find(string) {
                    Some(index) => {
                        // println!("{}", result)
                        let phrase = upper_text.get(index..(index + string.len())).unwrap();

                        let id = string_hash.iter().find_map(|(key, &ref val)|
                        if val == phrase {
                            Some(key)
                        } else {
                            None
                        }).unwrap();
                        
                        // println!("{:?} oh also {:?}", phrase, id);
                        phrases.insert(index.try_into().unwrap(), id.to_owned());
                    },
                    None => ()
                }
            }
        }
        return phrases;
    }

    pub fn string_to_id(string: String) -> i32 {
        let upper_char = &string.to_uppercase();
        let string_hash = get_hash();

        // iterate through values until we find a matching one, pass key (saves editing 2 different hashes)
        let potential_id = string_hash.iter().find_map(|(key, &ref val)|
        if val == upper_char {
            Some(key)
        } else {
            None
        });

        // check for \r and \n
        if potential_id.is_none() {
            return -1;
        }

        let id = potential_id.unwrap().to_owned();
        return id;
    }
}