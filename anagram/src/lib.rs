use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut input_word: HashMap<char, usize> = HashMap::new();
    let mut result: HashSet<&'a str> = HashSet::new();

    for c in word.chars().by_ref() {
        let lowercase_case = if c.is_uppercase() {
            c.to_lowercase().nth(0).unwrap()
        } else {
            c
        };
        if input_word.get(&lowercase_case).is_none() {
            input_word.insert(lowercase_case, 1);
        } else {
            *input_word.get_mut(&lowercase_case).unwrap() += 1;
        }
    }

    for s in possible_anagrams.iter() {
        if word == *s || word.to_lowercase() == s.to_lowercase() {
            continue;
        }

        let mut input_word_clone = input_word.clone();
        let mut iter = s.chars().peekable();

        loop {
            let item = iter.next();
            if item.is_none() {
                break;
            }

            let next_item = iter.peek();
            let char = item.unwrap();
            let lowercase_char = if char.is_uppercase() {
                char.to_lowercase().nth(0).unwrap()
            } else {
                char
            };

            if input_word_clone.get(&lowercase_char).is_none() {
                break;
            } else {
                *input_word_clone.get_mut(&lowercase_char).unwrap() -= 1;

                if *input_word_clone.get(&lowercase_char).unwrap() == 0 {
                    input_word_clone.remove(&lowercase_char);
                }
            }

            if input_word_clone.is_empty() && next_item.is_none() {
                result.insert(s);
            }
        }
    }

    result
}
