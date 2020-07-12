use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiplies: HashSet<u32> = HashSet::new();
    for i in factors {
        let mut currentMultiplyier: u32 = 1;

        while i * currentMultiplyier < limit {
            multiplies.insert(i * currentMultiplyier);
            currentMultiplyier += 1;
        }

    }
    multiplies.into_iter().fold(0, |acc, x| acc + x)
}
