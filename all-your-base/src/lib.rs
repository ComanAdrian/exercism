#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    let sum = number
        .iter()
        .rev()
        .enumerate()
        .map(|(i, num)| {
            if *num >= from_base {
                return Err(Error::InvalidDigit(*num));
            }
            Ok(from_base.pow(i as u32) * num)
        })
        .sum::<Result<u32, Error>>();

    match sum {
        Ok(sum) => {
            if sum == 0 {
                return Ok(vec![0]);
            }

            let mut final_digit_count = 0;
            loop {
                if sum < to_base.pow(final_digit_count) {
                    break;
                }
                final_digit_count += 1;
            }

            let mut result = vec![];
            let mut remainder = sum;
            for i in (0..final_digit_count).rev() {
                let mut coef = 0;

                loop {
                    if (coef) == to_base {
                        coef = 0;
                        break;
                    }
                    if ((coef + 1) * to_base.pow(i)) <= remainder {
                        coef += 1;
                    } else {
                        remainder -= coef * to_base.pow(i);
                        break;
                    }
                }

                result.push(coef);
            }

            Ok(result)
        }
        Err(err) => Err(err),
    }
}
