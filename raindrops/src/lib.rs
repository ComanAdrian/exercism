pub fn raindrops(num: i64) -> String {
    let outputs: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .into_iter()
        .filter(|(n, _)| num % n == 0)
        .map(|&(_, s)| s)
        .collect();
    if outputs.len() != 0 {
        outputs
    } else {
        num.to_string()
    }
}