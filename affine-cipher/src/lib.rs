use table::*;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const ALPHABET_LETTERS_COUNT: i32 = 26;

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if is_coprime(a) {
        return Err(AffineCipherError::NotCoprime(a));
    };

    let table = Table::new();
    let mut result = String::new();
    let encrypted_text = plaintext
        .chars()
        .filter(|c| !c.is_ascii_punctuation() && !c.is_whitespace())
        .map(|c| {
            if c.is_numeric() {
                c
            } else {
                let new_index = encryption(a, b, *table.get_index_by_char(&c).unwrap());
                table.get_char_by_index(new_index)
            }
        })
        .collect::<String>();

    for (i, c) in encrypted_text.chars().enumerate() {
        if i % 5 == 0 && i != 0 {
            result.push(' ')
        }
        result.push(c)
    }
    Ok(result)
}

fn encryption(a: i32, b: i32, x: usize) -> usize {
    (a as usize * x + b as usize) % ALPHABET_LETTERS_COUNT as usize
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if is_coprime(a) {
        return Err(AffineCipherError::NotCoprime(a));
    };

    let table = Table::new();

    let decrypted_text = ciphertext
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            if c.is_numeric() {
                c
            } else {
                let new_index = decryption(a, b, *table.get_index_by_char(&c).unwrap() as i32);
                table.get_char_by_index(new_index as usize)
            }
        })
        .collect::<String>();

    Ok(decrypted_text)
}

fn decryption(a: i32, b: i32, y: i32) -> i32 {
    // copied from solutions
    (mmi(a) * (y - b)).rem_euclid(ALPHABET_LETTERS_COUNT)
}

// copied from solutions
fn mmi(a: i32) -> i32 {
    for i in 0..ALPHABET_LETTERS_COUNT {
        if (a * i) % ALPHABET_LETTERS_COUNT == 1 {
            return i;
        }
    }
    1
}

fn is_coprime(a: i32) -> bool {
    let factors = get_factors();

    for i in factors.iter() {
        if a % i == 0 && *i != 1 {
            return true;
        }
    }

    return false;
}

fn get_factors() -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    for i in 1..=ALPHABET_LETTERS_COUNT {
        if ALPHABET_LETTERS_COUNT % i == 0 {
            result.push(i);
        }
    }
    result
}

//todo: you can use only `'a' as u32 + i` for getting a char
mod table {
    use std::collections::HashMap;

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
            self.by_char.get(&c.to_ascii_lowercase())
        }

        pub fn get_char_by_index(&self, index: usize) -> char {
            self.by_index[index]
        }
    }

    fn create_table_by_index() -> Vec<char> {
        let alphabet_letters_count = 26;
        let mut alphabet: Vec<char> = vec![];

        for i in 0..alphabet_letters_count {
            alphabet.push(std::char::from_u32('a' as u32 + i).unwrap());
        }

        alphabet
    }

    fn create_table_by_char() -> HashMap<char, usize> {
        let alphabet_letters_count = 26;
        let mut hash_map: HashMap<char, usize> = HashMap::new();

        for i in 0..alphabet_letters_count {
            hash_map.insert(std::char::from_u32('a' as u32 + i).unwrap(), i as usize);
        }

        hash_map
    }
}
