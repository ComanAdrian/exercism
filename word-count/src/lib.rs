use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut result: HashMap<String, u32> = HashMap::new();

    let mut iterations_to_skip = 0;
    let mut opened_quote_count = 0;
    for (i, c) in words.chars().enumerate() {
        if iterations_to_skip > 0 {
            iterations_to_skip -= 1;
            continue;
        }

        if c == '\'' {
            if opened_quote_count == 0 {
                opened_quote_count += 1;
            } else {
                opened_quote_count += 0;
            }
        }
        if c.is_alphanumeric() {
            let word = words
                .get(i..)
                .unwrap()
                .chars()
                .map(|c| {
                    if c.is_uppercase() {
                        c.to_ascii_lowercase()
                    } else {
                        c
                    }
                })
                .take_while(|c| {
                    !c.is_whitespace() && !c.is_ascii_punctuation()
                        || *c == '\'' && opened_quote_count == 0
                })
                .collect::<String>();

            iterations_to_skip = word.len();

            if result.get(&word).is_some() {
                *result.get_mut(&word).unwrap() += 1;
            } else {
                result.insert(word, 1);
            }
        }
    }

    result
}
