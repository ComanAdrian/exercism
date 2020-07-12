use direction::*;
use table::*;
pub fn encode(key: &str, s: &str) -> Option<String> {
    process(key, s, Direction::new(Mode::Encode))
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    process(key, s, Direction::new(Mode::Decode))
}

pub fn process(key: &str, s: &str, direction: Direction) -> Option<String> {
    if key.len() == 0 {
        return None;
    }
    let table: Table = Table::new();
    let mut result = String::new();
    let key_vec: Vec<char> = key.chars().collect();

    for (i, c) in s.chars().enumerate() {
        let char_index_in_table = table.get_index_by_char(&c).unwrap();

        let adjusted_key_index = if key_vec.len() <= i { 0 } else { i };

        match table.get_index_by_char(&key_vec[adjusted_key_index]) {
            Some(key_char_index) => {
                let rotate_to_index =
                    direction.rotate_to_index(char_index_in_table, key_char_index, table.len());
                result.push(table.get_char_by_index(rotate_to_index));
            }
            None => {
                result.clear();
                break;
            }
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    let table = Table::new();
    let mut key: String = String::new();

    for _ in 0..100 {
        let char = table.get_char_by_index(rand::random::<usize>() % 26);
        key.push(char);
    }
    (key.to_owned(), encode(&key, s).unwrap())
}

mod table {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Table {
        by_index: Vec<char>,
        by_char: HashMap<char, usize>,
    }

    impl Table {
        pub fn new() -> Table {
            Table {
                by_index: create_table_by_index(),
                by_char: create_table_by_char(),
            }
        }

        pub fn get_index_by_char(&self, c: &char) -> Option<&usize> {
            self.by_char.get(c)
        }

        pub fn get_char_by_index(&self, index: usize) -> char {
            self.by_index[index]
        }

        pub fn len(&self) -> usize {
            self.by_index.len()
        }
    }

    fn create_table_by_index() -> Vec<char> {
        let alphabet_letters_count = 26;
        let mut alphabet: Vec<char> = vec![];
        let a_letter = 'a' as u32;

        for i in 0..alphabet_letters_count {
            alphabet.push(std::char::from_u32(a_letter + i).unwrap());
        }

        alphabet
    }

    fn create_table_by_char() -> HashMap<char, usize> {
        let alphabet_letters_count = 26;
        let mut hash_map: HashMap<char, usize> = HashMap::new();
        let a_letter = 'a' as u32;

        for i in 0..alphabet_letters_count {
            hash_map.insert(std::char::from_u32(a_letter + i).unwrap(), i as usize);
        }

        hash_map
    }
}

mod direction {
    pub enum Mode {
        Encode,
        Decode,
    }

    pub struct Direction {
        mode: Mode,
    }

    impl Direction {
        pub fn new(mode: Mode) -> Direction {
            Direction { mode: mode }
        }

        pub fn rotate_to_index(&self, from: &usize, with: &usize, line_length: usize) -> usize {
            match self.mode {
                Mode::Encode => self.forward_to_index(from, with, line_length),
                Mode::Decode => self.backward_to_index(from, with, line_length),
            }
        }

        fn forward_to_index(&self, from: &usize, with: &usize, line_length: usize) -> usize {
            if (from + with) >= line_length {
                from + with - line_length
            } else {
                from + with
            }
        }

        fn backward_to_index(&self, from: &usize, with: &usize, line_length: usize) -> usize {
            if from >= with {
                from - with
            } else {
                from + line_length - with
            }
        }
    }
}
