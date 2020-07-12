use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut hash_set: HashSet<(u64, bool)> = HashSet::new();
    let mut result: Vec<u64> = Vec::new();

    for n in 2..upper_bound + 1 {
        match hash_set.get(&(n, true)) {
            Some(x) => {
                continue;
            }
            None => {
                hash_set.insert((n, false));
                result.push(n);
                let mut multiplier = 2;
                while n * multiplier < upper_bound + 1 {
                    hash_set.insert((n * multiplier, true));
                    multiplier += 1;
                }
            }
        }
    }
    result
}
