use std::collections::HashMap;

// ideally, would be great to verify more variants like
// 5 + 5 + 4 + 3... 5 + 4 + 4 + 4...
// not only having a strategy of starting to count with 5 or 4 groups
pub fn lowest_price(books: &[u32]) -> u32 {
    let vector: Vec<u32> = compute_ordered_vector(books);

    let five_group_strategy_basket_price =
        compute_total_price(books.len(), five_group_strategy(&vector));
    let four_group_strategy_basket_price =
        compute_total_price(books.len(), four_group_strategy(&vector));

    if five_group_strategy_basket_price < four_group_strategy_basket_price {
        five_group_strategy_basket_price
    } else {
        four_group_strategy_basket_price
    }
}

fn compute_ordered_vector(books: &[u32]) -> Vec<u32> {
    let mut hash_map: HashMap<u32, u32> = HashMap::new();
    books.into_iter().for_each(|v| match hash_map.get(v) {
        Some(_) => *hash_map.get_mut(v).unwrap() += 1,
        None => {
            hash_map.insert(*v, 1);
        }
    });
    let mut vector = hash_map.into_iter().map(|(_, v)| v).collect::<Vec<u32>>();
    vector.sort_by(|a, b| b.cmp(a));
    vector
}

fn compute_total_price(books_count: usize, aggregated_discount: f64) -> u32 {
    (books_count as f64 * 800.0
        - books_count as f64 * 800.0 * (aggregated_discount as f64) / books_count as f64) as u32
}

fn five_group_strategy(vector: &Vec<u32>) -> f64 {
    match try_to_compose_a_group(&vector, Grouping::Five.count()) {
        Some(dec_hash_map) => Grouping::Five.discount() + five_group_strategy(&dec_hash_map),
        None => four_group_strategy(&vector),
    }
}

fn four_group_strategy(vector: &Vec<u32>) -> f64 {
    match try_to_compose_a_group(&vector, Grouping::Four.count()) {
        Some(dec_vector) => Grouping::Four.discount() + four_group_strategy(&dec_vector),
        None => match try_to_compose_a_group(&vector, Grouping::Three.count()) {
            Some(dec_vector) => Grouping::Three.discount() + four_group_strategy(&dec_vector),
            None => match try_to_compose_a_group(&vector, Grouping::Two.count()) {
                Some(dec_vector) => Grouping::Two.discount() + four_group_strategy(&dec_vector),
                None => 0.0,
            },
        },
    }
}

fn can_compose_a_group(vector: &Vec<u32>, group_count: usize) -> bool {
    vector.into_iter().filter(|value| **value != 0).count() >= group_count
}

fn try_to_compose_a_group(vector: &Vec<u32>, group_count: usize) -> Option<Vec<u32>> {
    if can_compose_a_group(vector, group_count) {
        Some(
            vector
                .into_iter()
                .filter(|value| **value != 0)
                .enumerate()
                .map(
                    |(i, value)| {
                        if i >= group_count {
                            *value
                        } else {
                            *value - 1
                        }
                    },
                )
                .collect::<Vec<u32>>(),
        )
    } else {
        None
    }
}

enum Grouping {
    Five,
    Four,
    Three,
    Two,
}

impl Grouping {
    fn count(&self) -> usize {
        match &self {
            Grouping::Five => 5,
            Grouping::Four => 4,
            Grouping::Three => 3,
            Grouping::Two => 2,
        }
    }

    fn discount(&self) -> f64 {
        match &self {
            Grouping::Five => Grouping::Five.count() as f64 * 0.25,
            Grouping::Four => Grouping::Four.count() as f64 * 0.2,
            Grouping::Three => Grouping::Three.count() as f64 * 0.1,
            Grouping::Two => Grouping::Two.count() as f64 * 0.05,
        }
    }
}
