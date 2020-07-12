#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let dividend = num as f64;
    let mut current_sum = if dividend == 1.0 { 0.0 } else { 1.0 };
    let mut current_divisor = 2.0;
    let mut max_number = dividend;

    while current_divisor < max_number {
        let quotient = dividend / current_divisor;
        if quotient.fract() == 0.0 {
            current_sum += current_divisor;
            if current_divisor != quotient {
                current_sum += quotient;
            }
            max_number = quotient;
        }
        current_divisor += 1.0;
    }

    if current_sum > dividend {
        return Some(Classification::Abundant);
    } else if current_sum < dividend {
        return Some(Classification::Deficient);
    } else {
        return Some(Classification::Perfect);
    }
}
