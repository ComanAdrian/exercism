pub fn nth(n: u32) -> u32 {
    let mut current_number = 1;
    let mut prime_numbers: Vec<u32> = vec![];
    let mut is_not_prime: bool;

    loop {
        current_number += 1;
        is_not_prime = false;

        for prime_number in &prime_numbers {
            if (current_number % *prime_number) == 0 {
                is_not_prime = true;
                break;
            }
        }

        if (is_not_prime) {
            continue;
        }

        prime_numbers.push(current_number);

        if prime_numbers.len() - 1 == n as usize  {
            break
        }
    }

    return *prime_numbers.last().unwrap();
}
