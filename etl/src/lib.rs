use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result: Vec<(char, i32)> = Vec::new();

    for pair in h.into_iter() {

        for c in pair.1.into_iter() {
            result.push((c.to_lowercase().collect::<Vec<char>>()[0], *pair.0))
        }
    }

    result.into_iter().collect()
}
