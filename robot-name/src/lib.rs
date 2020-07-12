pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Robot::random_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::random_name();
    }

    fn random_name() -> String {
        format!(
            "{}{}{}{}{}",
            Robot::random_char(),
            Robot::random_char(),
            Robot::random_char_digit(),
            Robot::random_char_digit(),
            Robot::random_char_digit(),
        )
    }

    fn random_char() -> char {
        std::char::from_u32('A' as u32 + rand::random::<u32>() % 26).unwrap()
    }

    fn random_char_digit() -> char {
        std::char::from_u32('0' as u32 + rand::random::<u32>() % 9).unwrap()
    }
}
