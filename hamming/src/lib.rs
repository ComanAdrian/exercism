pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let count = s1
        .char_indices()
        .filter(|&tuple| tuple.1 != s2.chars().nth(tuple.0).unwrap())
        .count();

    Some(count)
}
