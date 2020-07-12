pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    if digits.len() < len {
        return result;
    }

    for n in 0..digits.len() - len + 1 {
        result.push(digits[n..n + len].to_string());
    }

    result
}
