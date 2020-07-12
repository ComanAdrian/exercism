use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut hash_set: HashSet<char> = HashSet::new();

    for mut c in candidate.chars() {
        if !c.is_alphabetic() {
            continue;
        }

        if c.is_ascii_uppercase() {
            c = c.to_ascii_lowercase()
        }

        if hash_set.contains(&c) {
            return false;
        } else {
            hash_set.insert(c);
        }
    }

    true
}
