pub fn is_armstrong_number(num: u32) -> bool {
    let mut result: u32 = 0;

    for c in num.to_string().chars() {
        result += c.to_digit(10).unwrap().pow(num.to_string().len() as u32)
    }

    result == num
}
