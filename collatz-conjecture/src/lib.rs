pub fn collatz(n: u64) -> Option<u64> {
    let mut result: Option<u64> = None;

    if n == 0 {
        return result;
    } else {
        result = Some(0);
    }

    let mut current_value: u64 = n;

    while current_value != 1 {
        match current_value % 2 {
            0 => current_value /= 2,
            _ => current_value = current_value * 3 + 1,
        }
        result = result.map(|v| v + 1);
    }

    result
}
