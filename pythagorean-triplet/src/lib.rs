use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result: HashSet<[u32; 3]> = HashSet::new();

    for a in 1..sum / 2 {
        for b in a..sum / 2 {
            let c = sum - a - b;

            if c < a || c < b {
                break;
            }

            if c.pow(2) - a.pow(2) == b.pow(2) {
                result.insert([a, b, c]);
            }
        }
    }

    result
}
