use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

// todo: very dirty solution, a lot of use-cases, refactor someday
#[derive(Debug)]
pub struct Decimal {
    value: String,
}

pub fn normalize_input(left: &str, right: &str) -> (String, String) {
    let left = if left.chars().nth(0).unwrap().to_digit(10).is_some() {
        left
    } else {
        left.get(1..).unwrap()
    };
    let right = if right.chars().nth(0).unwrap().to_digit(10).is_some() {
        right
    } else {
        right.get(1..).unwrap()
    };

    let left_splitted: Vec<&str> = left.split('.').collect();
    let left_decim = left_splitted.get(0).unwrap();
    let left_frac = match left_splitted.get(1) {
        Some(s) => s,
        None => "0",
    };

    let right_splitted: Vec<&str> = right.split('.').collect();
    let right_decim = right_splitted.get(0).unwrap();
    let right_frac = right_splitted.get(1).unwrap_or(&"");

    let mut new_left = String::new();
    let mut new_right = String::new();

    if left_decim.len() > right_decim.len() {
        let delta = left_decim.len() - right_decim.len();

        for i in 0..left_decim.len() {
            new_left.push(*left.as_bytes().get(i).unwrap() as char);

            if i < delta {
                new_right.push('0');
            } else {
                new_right.push(*right.as_bytes().get(i - delta).unwrap() as char);
            }
        }
    } else {
        let delta = right_decim.len() - left_decim.len();

        for i in 0..right_decim.len() {
            new_right.push(*right.as_bytes().get(i).unwrap() as char);

            if i < delta {
                new_left.push('0');
            } else {
                new_left.push(*left.as_bytes().get(i - delta).unwrap() as char);
            }
        }
    }

    new_left.push('.');
    new_right.push('.');

    new_left.push_str(left_frac);
    new_right.push_str(right_frac);

    if left_frac.len() > right_frac.len() {
        let delta = left_frac.len() - right_frac.len();

        for _ in 0..delta {
            new_right.push('0');
        }
    } else {
        let delta = right_frac.len() - left_frac.len();

        for _ in 0..delta {
            new_left.push('0');
        }
    }

    (new_left.to_owned(), new_right.to_owned())
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let self_first_char = self.value.chars().nth(0).unwrap();
        let rhs_first_char = rhs.value.chars().nth(0).unwrap();
        let (left, right) = normalize_input(&self.value, &rhs.value);
        let unsigned_self = Decimal::try_from(&left).unwrap();
        let unsigned_rhs = Decimal::try_from(&right).unwrap();

        match (self_first_char, rhs_first_char) {
            ('-', '-') => {
                return (unsigned_self + unsigned_rhs).prepend_sign("-");
            }
            (_, '-') => {
                if unsigned_self > unsigned_rhs {
                    return unsigned_self - unsigned_rhs;
                } else {
                    return (unsigned_rhs - unsigned_self).prepend_sign("-");
                }
            }
            ('-', _) => {
                if unsigned_self > unsigned_rhs {
                    return (unsigned_self - unsigned_rhs).prepend_sign("-");
                } else {
                    return unsigned_rhs - unsigned_self;
                }
            }
            _ => {}
        }

        let (left, right) = normalize_input(&self.value, &rhs.value);
        let mut left_iter = left.chars().rev();
        let mut right_iter = right.chars().rev();

        let mut result = String::new();
        let mut carry = 0;
        let mut skip_frac_zeros = true;
        loop {
            let left_next = left_iter.next();
            let right_next = right_iter.next();

            match left_next {
                Some(v) => {
                    if v == '.' {
                        skip_frac_zeros = false;
                        if !result.is_empty() {
                            result.push('.');
                        }
                    } else {
                        let mut sum = (v.to_digit(10).unwrap()
                            + right_next.unwrap().to_digit(10).unwrap())
                            + carry;

                        if sum > 9 {
                            carry = 1;
                            sum -= 10;
                        } else {
                            carry = 0;
                        }
                        if skip_frac_zeros && sum == 0 {
                            continue;
                        } else {
                            skip_frac_zeros = false;
                            result.push(std::char::from_u32('0' as u32 + sum).unwrap());
                        }
                    }
                }
                None => {
                    if carry != 0 {
                        result.push(std::char::from_u32('0' as u32 + carry).unwrap());
                    }
                    break;
                }
            }
        }
        result = result.chars().rev().collect::<String>();

        let result_splitted: Vec<&str> = result.split('.').collect();
        let result_dec = result_splitted.get(0).unwrap();

        if result_splitted.get(1).is_none() {
            let first_char = left.chars().nth(0).unwrap();
            if (first_char == '1' || first_char == '0') && result_dec.len() == 1 {
                result.push_str(".0");
            }
        }

        if result.is_empty() {
            Decimal {
                value: "0".to_owned(),
            }
        } else {
            Decimal { value: result }
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let self_first_char = self.value.chars().nth(0).unwrap();
        let rhs_first_char = rhs.value.chars().nth(0).unwrap();
        let (left, right) = normalize_input(&self.value, &rhs.value);
        let unsigned_self = Decimal::try_from(&left).unwrap();
        let unsigned_rhs = Decimal::try_from(&right).unwrap();

        match (self_first_char, rhs_first_char) {
            ('-', _) => {
                return (unsigned_self + unsigned_rhs).prepend_sign("-");
            }
            (_, '-') => {
                return unsigned_self + unsigned_rhs;
            }
            _ => {
                if unsigned_self < unsigned_rhs {
                    return (unsigned_self + unsigned_rhs).prepend_sign("-");
                }
            }
        }

        let mut left_iter = left.chars().rev();
        let mut right_iter = right.chars().rev();

        let mut result = String::new();
        let mut carry = 0;
        let mut skip_frac_zeros = true;
        loop {
            let left_next = left_iter.next();
            let right_next = right_iter.next();

            match left_next {
                Some(v) => {
                    if v == '.' {
                        if !result.is_empty() {
                            result.push('.');
                        }
                    } else {
                        let left_digit = v.to_digit(10).unwrap();
                        let left_digit = if left_digit >= carry {
                            let res = left_digit - carry;
                            carry = 0;
                            res
                        } else {
                            left_digit + 10 - carry
                        };

                        let right_digit = right_next.unwrap().to_digit(10).unwrap();

                        let diff = if left_digit >= right_digit {
                            left_digit - right_digit
                        } else {
                            carry += 1;
                            left_digit + 10 - right_digit
                        };

                        if skip_frac_zeros && diff == 0 {
                            continue;
                        } else {
                            skip_frac_zeros = false;
                            result.push(std::char::from_u32('0' as u32 + diff).unwrap());
                        }
                    }
                }
                None => {
                    if carry != 0 {
                        result.push(std::char::from_u32('0' as u32 + carry).unwrap());
                    }
                    break;
                }
            }
        }
        result = result.chars().rev().collect::<String>();

        if result.split('.').collect::<Vec<&str>>().get(1).is_none() {
            let first_char = left.chars().nth(0).unwrap();
            if first_char == '1' || first_char == '0' {
                result.push_str(".0");
            }
        }
        Decimal { value: result }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (left, right) = normalize_input(&self.value, &rhs.value);

        let mut result: Vec<String> = vec![];
        let mut carry = 0;

        let mut point_index_correction = 0;
        for (l_i, l_c) in left.chars().rev().enumerate() {
            if l_c == '.' {
                point_index_correction = 1;
                continue;
            }

            if result.get(l_i - point_index_correction).is_none() {
                result.push(String::new())
            }

            for _ in 0..l_i - point_index_correction {
                result
                    .get_mut(l_i - point_index_correction)
                    .unwrap()
                    .push('0')
            }

            for r_c in right.chars().rev() {
                if r_c == '.' {
                    continue;
                }
                let product = l_c.to_digit(10).unwrap() * r_c.to_digit(10).unwrap() + carry;
                carry = product / 10;
                let diff = product % 10;

                result
                    .get_mut(l_i - point_index_correction)
                    .unwrap()
                    .push(std::char::from_u32('0' as u32 + diff).unwrap());
            }
        }

        let mut pointless_decimal_vec = result
            .iter()
            .map(|s| s.chars().rev().collect::<String>())
            .map(|s| Decimal::try_from(&s).unwrap())
            .fold(Decimal::try_from("0.0").unwrap(), |acc, x| acc + x)
            .value
            .chars()
            .collect::<Vec<char>>();

        let left_splitted = left.split('.').collect::<Vec<&str>>();
        let right_splitted = right.split('.').collect::<Vec<&str>>();
        let left_frac = left_splitted.get(1).unwrap();
        let right_frac = right_splitted.get(1).unwrap();
        pointless_decimal_vec.insert(
            pointless_decimal_vec.len() - (left_frac.len() + right_frac.len()),
            '.',
        );

        let mut should_skip_frac = true;
        let trim_end_value = pointless_decimal_vec
            .iter()
            .rev()
            .filter(|&&c| {
                if c == '0' && should_skip_frac {
                    false
                } else {
                    should_skip_frac = false;
                    true
                }
            })
            .rev()
            .collect::<String>();
        let decimal = Decimal {
            value: trim_end_value,
        };

        let self_first_char = self.value.chars().nth(0).unwrap();
        let rhs_first_char = rhs.value.chars().nth(0).unwrap();
        match (self_first_char, rhs_first_char) {
            ('-', '-') => decimal,
            ('-', _) => decimal.prepend_sign("-"),
            (_, '-') => decimal.prepend_sign("-"),
            _ => decimal,
        }
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let self_first_char = self.value.chars().nth(0).unwrap();
        let rhs_first_char = other.value.chars().nth(0).unwrap();
        let (self_normalized, other_normalized) = normalize_input(&self.value, &other.value);

        match (self_first_char, rhs_first_char) {
            ('-', '-') => self_normalized == other_normalized,
            ('-', _) => false,
            (_, '-') => false,
            _ => self_normalized == other_normalized,
        }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_first_char = self.value.chars().nth(0).unwrap();
        let other_first_char = other.value.chars().nth(0).unwrap();

        let (self_normalized, other_normalized) = normalize_input(&self.value, &other.value);
        match (self_first_char, other_first_char) {
            ('-', '-') => other_normalized.partial_cmp(&self_normalized),
            (_, '-') => Some(Ordering::Greater),

            _ => self_normalized.partial_cmp(&other_normalized),
        }
    }
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        Some(Decimal {
            value: input.to_string(),
        })
    }

    fn prepend_sign(&self, sign: &str) -> Decimal {
        let mut res = sign.to_owned();
        res.push_str(&self.value);
        Decimal { value: res }
    }
}
