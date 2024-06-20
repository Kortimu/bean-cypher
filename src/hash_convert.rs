pub mod hash_convert {
    use std::collections::HashMap;

    lazy_static! {
        static ref CHAR_HASH: HashMap<i32, char> = HashMap::from([
            (0, '0'),
            (1, '1'),
            (2, '2'),
            (3, '3'),
            (4, '4'),
            (5, '5'),
            (6, '6'),
            (7, '7'),
            (8, '8'),
            (9, '9'),
            (10, ' '),
            (11, 'A'),
            (12, 'Ā'),
            (13, 'B'),
            (14, 'C'),
            (15, 'Č'),
            (16, 'D'),
            (17, 'E'),
            (18, 'Ē'),
            (19, 'F'),
            (20, 'G'),
            (21, 'Ģ'),
            (22, 'H'),
            (23, 'I'),
            (24, 'Ī'),
            (25, 'J'),
            (26, 'K'),
            (27, 'Ķ'),
            (28, 'L'),
            (29, 'Ļ'),
            (30, 'M'),
            (31, 'N'),
            (32, 'Ņ'),
            (33, 'O'),
            (34, 'P'),
            (35, 'Q'),
            (36, 'R'),
            (37, 'S'),
            (38, 'Š'),
            (39, 'T'),
            (40, 'U'),
            (41, 'Ū'),
            (42, 'V'),
            (43, 'W'),
            (44, 'X'),
            (45, 'Y'),
            (46, 'Z'),
            (47, 'Ž'),
            (48, '!'),
            (49, '"'),
            (50, '#'),
            (51, '$'),
            (52, '%'),
            (53, '&'),
            (54, '\''),
            (55, '('),
            (56, ')'),
            (57, '*'),
            (58, '+'),
            (59, ','),
            (60, '-'),
            (61, '.'),
            (62, '/'),
            (63, ':'),
            (64, ';'),
            (65, '<'),
            (66, '='),
            (67, '>'),
            (68, '?'),
            (69, '@'),
            (70, '['),
            (71, '\\'),
            (72, ']'),
            (73, '^'),
            (74, '_'),
            (75, '`'),
            (76, '{'),
            (77, '|'),
            (78, '}'),
            (79, '~')
        ]);

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

    pub fn print_from_id(id: i32) {
        match STRING_HASH.get(&id) {
            Some(result) => print!("{}", result),
            None => ()
        }
    }

    pub fn char_to_id(char: char) -> Vec<i32> {
        // converting a char to uppercase, it can become 2 chars with some unicode chars (e.g. ﬁ => ['F', 'I']). i love standards
        let upper_chars = char.to_uppercase().collect::<Vec<_>>();
        let mut keys = Vec::new();

        for upper_char in upper_chars {
            // iterate through values until we find a matching one, pass key (saves editing 2 different hashes)
            let uhhh = CHAR_HASH.iter().find_map(|(key, &val)|
            if val == upper_char {
                Some(key)
            } else {
                None
            });

            // check for \r and \n (currently not testing for cursed nonsense)
            if uhhh.is_none() {
                return keys;
            }

            let hmm = uhhh.unwrap().to_owned();
            keys.push(hmm);
        }
        return keys;
    }
}