pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        if self.to_string().trim().len() <= 1 {
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
            .to_string()
            .chars()
            .any(|c| c.is_alphabetic() || c.is_ascii_punctuation());

        if is_invalid_string {
            return false;
        }

        self.to_string()
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
