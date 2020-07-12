pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut count: u32 = 0;
    let mut iter = source.chars().peekable();

    for _ in 0..source.len() {
        let char = iter.next();
        let next_char = iter.peek();
        count += 1;

        if next_char != char.as_ref() {
            if count > 1 {
                result.push_str(count.to_string().as_str())
            }
            result.push(char.unwrap());
            count = 0;
        }
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut iter = source.chars().peekable();

    while let Some(char) = iter.next() {
        if char.is_numeric() {
            let mut number = String::new();
            number.push(char);

            let mut inner_iter = iter.clone();
            while inner_iter.peek().unwrap().is_numeric() {
                number.push(inner_iter.next().unwrap());
                iter.next();
            }

            for _ in 0..number.parse().unwrap() {
                result.push(iter.peek().unwrap().to_owned())
            }
            iter.next();
        } else {
            result.push(char)
        }
    }

    result
}
