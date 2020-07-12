use std::collections::HashMap;

pub fn score(word: &str) -> u64 {
    let scrabble_table = make_scrabble_table();

    word.chars()
        .map(|mut v| {
            if v.is_lowercase() {
                v = v.to_ascii_uppercase()
            }

            if let Some(v) = scrabble_table.get(&v) {
                return *v;
            } else {
                return 0;
            }
        })
        .sum::<u64>()
}

pub fn make_scrabble_table() -> HashMap<char, u64> {
    let mut result: HashMap<char, u64> = HashMap::new();
    let values: Vec<(Vec<char>, u64)> = vec![
        (vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'], 1),
        (vec!['D', 'G'], 2),
        (vec!['B', 'C', 'M', 'P'], 3),
        (vec!['F', 'H', 'V', 'W', 'Y'], 4),
        (vec!['K'], 5),
        (vec!['J', 'X'], 8),
        (vec!['Q', 'Z'], 10),
    ];

    values.iter().for_each(|tuple| {
        tuple.0.iter().for_each(|&char| {
            result.insert(char, tuple.1);
        });
    });

    result
}
