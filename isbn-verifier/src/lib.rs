/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.len() != 10 && isbn.len() != 13 {
        return false;
    }

    let sum = isbn
        .chars()
        .rev()
        .filter(|c| *c != '-')
        .enumerate()
        .map(|(i, c)| {
            if c == 'X' && i == 0 {
                Some(10)
            } else {
                c.to_digit(10)
            }
        })
        .enumerate()
        .map(|(i, n)| match n {
            Some(num) => Some(num * (i as u32 + 1)),
            None => None,
        })
        .sum::<Option<u32>>();

    match sum {
        Some(sum) => {
            if sum % 11 == 0 {
                true
            } else {
                false
            }
        }
        None => false,
    }
}
