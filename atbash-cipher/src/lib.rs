use std::collections::HashMap;

const PLAIN: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const CIPHER: [char; 26] = [
    'z', 'y', 'x', 'w', 'v', 'u', 't', 's', 'r', 'q', 'p', 'o', 'n', 'm', 'l', 'k', 'j', 'i', 'h',
    'g', 'f', 'e', 'd', 'c', 'b', 'a',
];

const GROUP_SIZE: usize = 5;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut result = String::new();
    let letters_map = create_letters_map();
    let mut result_alphanumeric_length = 0;

    for (_, c) in plain.chars().enumerate() {
        if !c.is_alphanumeric() {
            continue;
        }

        if result_alphanumeric_length != 0 && result_alphanumeric_length % GROUP_SIZE == 0 {
            result.push(' ');
        }
        result_alphanumeric_length += 1;

        let lowercase_char = if c.is_uppercase() {
            c.to_lowercase().collect::<String>().chars().nth(0).unwrap()
        } else {
            c
        };

        if lowercase_char.is_alphabetic() {
            result.push(letters_map.get(&lowercase_char).unwrap().to_owned());
        } else {
            result.push(lowercase_char);
        }
    }

    result
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut result: String = String::new();
    let hashmap = create_letters_map();

    for (_, c) in cipher.chars().enumerate() {
        if c.is_whitespace() {
            continue;
        }

        if c.is_alphabetic() {
            result.push(hashmap.get(&c).unwrap().to_owned());
        } else {
            result.push(c);
        }
    }

    result
}

fn create_letters_map() -> HashMap<char, char> {
    let mut result: HashMap<char, char> = HashMap::new();

    for (i, _) in PLAIN.iter().enumerate() {
        result.insert(
            PLAIN.get(i).unwrap().to_owned(),
            CIPHER.get(i).unwrap().to_owned(),
        );
    }
    result
}
