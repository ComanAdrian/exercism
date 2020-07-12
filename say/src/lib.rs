use std::collections::HashMap;

const HIGHEST_U64_ORDER: u32 = 19;

macro_rules! hash_map {
    ( $z:ident, $(($x:expr, $y:expr)), * ) => {
        {
            let mut result: HashMap<$z, &str> = HashMap::new();

            $(
                result.insert($x, $y);
            )*
            result
        }
    };
}

pub fn encode(n: u64) -> String {
    let encoder = Encoder::new(n);

    encoder.encode()
}

struct Encoder {
    number: u64,
}

// TODO: refactor
impl Encoder {
    fn new(number: u64) -> Self {
        Encoder { number }
    }

    pub fn encode(&self) -> String {
        if self.number == 0 {
            return "zero".to_owned();
        }
        let mut result: String = String::new();
        let mut remainder = self.number;

        let total_digits = self.calc_number_of_digits();
        let mut order = total_digits - total_digits % 3;
        let mut three_digits_block = self.number / (10_u64.pow(order) as u64);

        loop {
            result += Encoder::encode_three_digits_block(three_digits_block, order).as_str();
            remainder -= three_digits_block * (10_u64.pow(order) as u64);

            if order == 0 {
                break;
            }
            order -= 3;
            three_digits_block = remainder / (10_u64.pow(order) as u64);
        }

        result.trim_start().to_owned()
    }

    fn calc_number_of_digits(&self) -> u32 {
        let mut result = 0;
        loop {
            result += 1;
            if result == HIGHEST_U64_ORDER || self.number <= 10_u64.pow(result) as u64 {
                return result;
            }
        }
    }

    fn encode_three_digits_block(n: u64, order: u32) -> String {
        let order_of_magnitude = orders_of_magnitude();
        let numbers = numbers();
        let mut result: String = String::new();
        let hundreds_count = n / 100;
        let mut remainder = n;

        if remainder > 0 {
            result.push_str(" ");
        }

        if hundreds_count > 0 {
            remainder -= hundreds_count * 100;

            result.push_str(numbers.get(&hundreds_count).unwrap());
            result.push_str(" hundred");
        }

        if remainder == 0 {
            return result;
        }

        if hundreds_count > 0 {
            result.push(' ');
        }

        if remainder < 20 {
            result.push_str(numbers.get(&remainder).unwrap());
        } else {
            let decimals_count = remainder / 10;

            result.push_str(numbers.get(&(decimals_count * 10)).unwrap());
            remainder -= decimals_count * 10;

            if remainder != 0 {
                result.push_str("-");
                result.push_str(numbers.get(&remainder).unwrap());
            }
        }

        if order != 0 {
            result.push(' ');
            result.push_str(order_of_magnitude.get(&order).unwrap());
        }

        result
    }
}

fn numbers<'a>() -> HashMap<u64, &'a str> {
    hash_map! {
        u64,
        (0, "zero"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety")
    }
}

fn orders_of_magnitude<'a>() -> HashMap<u32, &'a str> {
    hash_map! {
        u32,
        (0, ""),
        (3, "thousand"),
        (6, "million"),
        (9, "billion"),
        (12, "trillion"),
        (15, "quadrillion"),
        (18, "quintillion")
    }
}
