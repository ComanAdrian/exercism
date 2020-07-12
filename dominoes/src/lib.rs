fn search_for_chain(
    first: (u8, u8),
    current: (u8, u8),
    pool: &[(u8, u8)],
) -> Option<Vec<(u8, u8)>> {
    if pool.is_empty() {
        if first.0 == current.1 {
            return Some(vec![current]);
        } else {
            return None;
        }
    } else {
        let mut next_candidates = pool
            .iter()
            .filter(|dom| current.1 == dom.0 || current.1 == dom.1);

        next_candidates.find_map(|&next| {
            let next_candidate = if current.1 == next.0 {
                next
            } else {
                (next.1, next.0)
            };

            let rest = pool
                .iter()
                .take_while(|d| **d != next)
                .chain(pool.iter().skip_while(|d| **d != next).skip(1))
                .map(|&d| d)
                .collect::<Vec<(u8, u8)>>();

            let forward_chain = search_for_chain(first, next_candidate, &rest);
            if let Some(mut chain) = forward_chain {
                chain.insert(0, current);
                return Some(chain);
            } else {
                return None;
            }
        })
    }
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if let Some(first) = input.first() {
        search_for_chain(*first, *first, &input[1..])
    } else {
        Some(vec![])
    }
}
