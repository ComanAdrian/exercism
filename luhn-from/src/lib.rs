pub struct Luhn {
    value: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.value.trim().len() <= 1 {
            return false;
        };

        let adjust_digit = |(i, n)| -> u32 {
            if i % 2 == 0 {
                n
            } else {
                let new_number = n * 2;
                if new_number >= 10 {
                    new_number - 9
                } else {
                    new_number
                }
            }
        };

        let is_invalid_string = self
            .value
            .chars()
            .any(|c| c.is_alphabetic() || c.is_ascii_punctuation());

        if is_invalid_string {
            return false;
        }

        self.value
            .chars()
            .filter_map(|c| c.to_digit(10))
            .rev()
            .enumerate()
            .map(adjust_digit)
            .sum::<u32>()
            % 10
            == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?

impl<T: ToString + Sized> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            value: input.to_string(),
        }
    }
}
