use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Palindrome {
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        self.factors[0].0 * self.factors[0].1
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b))
    }
}

impl PartialEq for Palindrome {
    fn eq(&self, other: &Palindrome) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Palindrome {
    fn partial_cmp(&self, other: &Palindrome) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_palindrome: Option<Palindrome> = None;
    let mut max_palindrome: Option<Palindrome> = None;

    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if is_palindrome(product) {
                let palindrome = Palindrome::new(i, j);
                if min_palindrome.is_none() || max_palindrome.is_none() {
                    min_palindrome = Some(palindrome.clone());
                    max_palindrome = Some(palindrome.clone());
                    continue;
                }

                if palindrome == *min_palindrome.as_ref().unwrap() {
                    min_palindrome.as_mut().unwrap().insert(i, j);
                } else if palindrome == *max_palindrome.as_ref().unwrap() {
                    max_palindrome.as_mut().unwrap().insert(i, j);
                }

                if palindrome < *min_palindrome.as_ref().unwrap() {
                    min_palindrome = Some(palindrome.clone())
                } else if palindrome > *max_palindrome.as_ref().unwrap() {
                    max_palindrome = Some(palindrome.clone())
                }
            }
        }
    }

    if min_palindrome.is_none() {
        None
    } else {
        Some((min_palindrome.unwrap(), max_palindrome.unwrap()))
    }
}

pub fn is_palindrome(number: u64) -> bool {
    let string = number.to_string();
    let mut regular_iterator = string.chars();
    let mut reverse_iterator = string.chars().rev();

    for _ in 0..=string.len() / 2 {
        if regular_iterator.next() != reverse_iterator.next() {
            return false;
        }
    }

    true
}
