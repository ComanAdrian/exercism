use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let mut alphabet = make_alphabet();

    for i in sentence.chars() {
        alphabet.remove(&i.to_ascii_lowercase());
    }

    alphabet.is_empty()
}

fn make_alphabet() -> HashSet<char> {
    let mut result: HashSet<char> = HashSet::new();
    let alphabet = "The quick brown fox jumps over the lazy dog";

    alphabet.chars().for_each(|c| {
        if c.is_ascii_alphabetic() {
            result.insert(c.to_ascii_lowercase());
        }
    });

    result
}
