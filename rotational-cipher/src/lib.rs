use std::collections::HashMap;

const PLAIN: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn rotate(input: &str, key: i8) -> String {
    let mut result: String = String::new();
    let letters_mapping = create_letters_mapping();

    for c in input.chars() {
        let encrypted_char = if !c.is_alphabetic() {
            c
        } else {
            if c.is_uppercase() {
                let lowercase_char = c.to_lowercase().collect::<String>().chars().nth(0).unwrap();
                PLAIN[calculate_plain_index(&letters_mapping, &lowercase_char, key)]
                    .to_uppercase()
                    .collect::<String>()
                    .chars()
                    .nth(0)
                    .unwrap()
            } else {
                PLAIN[calculate_plain_index(&letters_mapping, &c, key)]
            }
        };

        result.push(encrypted_char);
    }

    result
}

fn calculate_plain_index(letters_mapping: &HashMap<char, i8>, c: &char, key: i8) -> usize {
    let plain_index = (*letters_mapping.get(c).unwrap() + key) % PLAIN.len() as i8;
    let result = if plain_index < 0 {
        PLAIN.len() as i8 + plain_index
    } else {
        plain_index
    };
    result as usize
}

fn create_letters_mapping() -> HashMap<char, i8> {
    let mut result: HashMap<char, i8> = HashMap::new();

    for (i, _) in PLAIN.iter().enumerate() {
        result.insert(PLAIN.get(i).unwrap().to_owned(), i as i8);
    }
    result
}
