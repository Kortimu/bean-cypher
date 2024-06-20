pub mod hash_convert {
    use std::collections::HashMap;

    lazy_static! {
        static ref STRING_HASH: HashMap<i32, &'static str> = HashMap::from([
            (0, "0"),
            (1, "1"),
            (2, "2"),
            (3, "3"),
            (4, "4"),
            (5, "5"),
            (6, "6"),
            (7, "7"),
            (8, "8"),
            (9, "9"),
            (10, " "),
            (11, "A"),
            (12, "Ā"),
            (13, "B"),
            (14, "C"),
            (15, "Č"),
            (16, "D"),
            (17, "E"),
            (18, "Ē"),
            (19, "F"),
            (20, "G"),
            (21, "Ģ"),
            (22, "H"),
            (23, "I"),
            (24, "Ī"),
            (25, "J"),
            (26, "K"),
            (27, "Ķ"),
            (28, "L"),
            (29, "Ļ"),
            (30, "M"),
            (31, "N"),
            (32, "Ņ"),
            (33, "O"),
            (34, "P"),
            (35, "Q"),
            (36, "R"),
            (37, "S"),
            (38, "Š"),
            (39, "T"),
            (40, "U"),
            (41, "Ū"),
            (42, "V"),
            (43, "W"),
            (44, "X"),
            (45, "Y"),
            (46, "Z"),
            (47, "Ž"),
            (48, "!"),
            (49, "\'"),
            (50, "#"),
            (51, "$"),
            (52, "%"),
            (53, "&"),
            (54, "\""),
            (55, "("),
            (56, ")"),
            (57, "*"),
            (58, "+"),
            (59, ","),
            (60, "-"),
            (61, "."),
            (62, "/"),
            (63, ":"),
            (64, ";"),
            (65, "<"),
            (66, "="),
            (67, ">"),
            (68, "?"),
            (69, "@"),
            (70, "["),
            (71, "\\"),
            (72, "]"),
            (73, "^"),
            (74, "_"),
            (75, "`"),
            (76, "{"),
            (77, "|"),
            (78, "}"),
            (79, "~")
        ]);
    }

    pub fn id_to_string(id: i32) -> String {
        match STRING_HASH.get(&id) {
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

        for string in STRING_HASH.values() {
            // those damn funny unicode characters!! (ā, ē, ū and the like)
            if string.len() > 2 {
                match upper_text.find(string) {
                    Some(index) => {
                        // println!("{}", result)
                        let phrase = upper_text.get(index..(index + string.len())).unwrap();

                        let id = STRING_HASH.iter().find_map(|(key, &val)|
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
        let upper_char = string.to_uppercase();

        // iterate through values until we find a matching one, pass key (saves editing 2 different hashes)
        let potential_id = STRING_HASH.iter().find_map(|(key, &val)|
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