pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let mut number: u64 = n;
    let mut currentDivider: u64 = 2;

    if number == 1 {
        return result;
    }

    loop {
        if number == currentDivider {
            result.push(currentDivider);
            break;
        }

        if number % currentDivider == 0 {
            number = number / currentDivider;
            result.push(currentDivider);
            currentDivider = 2;
            continue;
        } else {
            currentDivider += 1;
        }
    }

    result
}
