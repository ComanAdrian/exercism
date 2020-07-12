pub fn abbreviate(phrase: &str) -> String {
    let char_vec: Vec<char> = phrase.chars().collect();
    let mut result: Vec<char> = Vec::new();

    for c_tuple in phrase.chars().enumerate() {
        if !c_tuple.1.is_alphabetic() {
            continue;
        }

        if c_tuple.0 == 0
            || char_vec[c_tuple.0 - 1].is_whitespace()
            || c_tuple.1.is_uppercase() && char_vec[c_tuple.0 - 1].is_lowercase()
            || ['-', '_'].contains(&char_vec[c_tuple.0 - 1])
        {
            if c_tuple.1.is_uppercase() {
                result.push(c_tuple.1);
            } else {
                result.push(c_tuple.1.to_uppercase().collect::<Vec<char>>()[0]);
            }
        }
    }

    result
        .into_iter()
        .map(|c| c.to_string())
        .collect::<String>()
}
