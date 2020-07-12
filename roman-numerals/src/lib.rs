use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let conversion_table = create_conversion_table();
        let mut remainder = num;
        let mut result = String::new();

        while remainder != 0 {
            for record in conversion_table.iter() {
                let Record {
                    eng_numeral,
                    roman_numeral,
                    eng_numeral_prefix,
                    roman_numeral_prefix,
                } = *record;
                let symbol_count = count_roman_symbols(remainder, eng_numeral);

                if symbol_count == 0 {
                    match eng_numeral_prefix {
                        Some(num) => {
                            let symbol_count_in_prefix_case =
                                count_roman_symbols(remainder + num, eng_numeral);

                            if symbol_count_in_prefix_case == 1 {
                                result.push(roman_numeral_prefix.unwrap());
                                result.push(roman_numeral);
                                remainder = remainder + num - eng_numeral;
                            }
                        }
                        _ => {}
                    }
                } else {
                    remainder = if symbol_count != 0 {
                        remainder % eng_numeral
                    } else {
                        remainder
                    };

                    for _ in 0..symbol_count {
                        result.push(roman_numeral);
                    }
                }

                println!(
                    "symbol_count for: {}, is: {}, remainder: {}, result: {:?}",
                    roman_numeral, symbol_count, remainder, result
                );
            }
        }

        Roman(result)
    }
}

fn count_roman_symbols(num: u32, roman_char_numeral: u32) -> u32 {
    (num - num % roman_char_numeral) / roman_char_numeral
}

macro_rules! records {
    ( $(($eng_numeral:expr, $roman_numeral:expr, $eng_numeral_prefix:expr, $roman_numeral_prefix:expr)), * ) => {
        vec![
        $(
            Record {
                eng_numeral: $eng_numeral,
                roman_numeral: $roman_numeral,
                eng_numeral_prefix: $eng_numeral_prefix,
                roman_numeral_prefix: $roman_numeral_prefix
            },
        )*
        ]
    };
}

fn create_conversion_table() -> Vec<Record> {
    records! {
        (1000, 'M', Some(100), Some('C')),
        (500, 'D', Some(100), Some('C')),
        (100, 'C', Some(10), Some('X')),
        (50, 'L', Some(10), Some('X')),
        (10, 'X', Some(1), Some('I')),
        (5, 'V', Some(1), Some('I')),
        (1, 'I', None, None)
    }
}

struct Record {
    eng_numeral: u32,
    roman_numeral: char,
    eng_numeral_prefix: Option<u32>,
    roman_numeral_prefix: Option<char>,
}
