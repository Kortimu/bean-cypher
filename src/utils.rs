pub mod hash_convert {
    use std::collections::HashMap;

    lazy_static! {
        static ref CHAR_HASH: HashMap<i32, char> = HashMap::from([
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
    }

    pub fn print_from_id(id: i32) {
        match CHAR_HASH.get(&id) {
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