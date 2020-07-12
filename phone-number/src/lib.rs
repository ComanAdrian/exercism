// todo: Encapsulate into a struct
pub fn number(user_number: &str) -> Option<String> {
    let mut normalized_user_number = user_number
        .chars()
        .filter(|c| c.to_digit(10).is_some())
        .collect::<String>();

    if has_country_code(&normalized_user_number) {
        let (country_code, nanp_number) = normalized_user_number.split_at(1);
        if !is_valid_country_code(country_code) {
            return None;
        }

        normalized_user_number = nanp_number.to_owned();
    }

    if !is_valid_length(&normalized_user_number)
        || !is_valid_area_code(&normalized_user_number)
        || !is_valid_exchange_code(&normalized_user_number)
    {
        return None;
    }

    return Some(normalized_user_number);
}

fn is_valid_length(user_number: &str) -> bool {
    user_number.len() == 10
}

fn has_country_code(user_number: &str) -> bool {
    user_number.len() == 11
}

fn is_valid_country_code(country_code: &str) -> bool {
    country_code == "1"
}

fn is_valid_area_code(user_number: &str) -> bool {
    user_number.chars().nth(0).unwrap().to_digit(10).unwrap() >= 2
}

fn is_valid_exchange_code(user_number: &str) -> bool {
    user_number.chars().nth(3).unwrap().to_digit(10).unwrap() >= 2
}

// mod user_number_validator {
//     struct UserNumberValidator {
//         user_number: String,
//     }
//
//     impl UserNumberValidator {
//         fn new(user_number: &str) -> UserNumberValidator {
//             let normalized_user_number = user_number
//                 .chars()
//                 .filter(|c| c.to_digit(10).is_some())
//                 .collect::<String>();
//
//             UserNumberValidator {
//                 user_number: normalized_user_number,
//             }
//         }
//
//         fn is_valid(&self) -> Option<String> {
//             if !self.is_valid_length()
//                 || !self.is_valid_area_code()
//                 || !self.is_valid_exchange_code()
//             {
//                 return None;
//             }
//
//             return Some(self.user_number.to_owned());
//         }
//
//         fn is_valid_length(&self) -> bool {
//             self.user_number.len() == 10
//         }
//
//         fn has_country_code(&self) -> bool {
//             self.user_number.len() == 11
//         }
//
//         // fn is_valid_country_code(&self) -> bool {
//         //     country_code == "1"
//         // }
//
//         fn is_valid_area_code(&self) -> bool {
//             self.user_number
//                 .chars()
//                 .nth(0)
//                 .unwrap()
//                 .to_digit(10)
//                 .unwrap()
//                 >= 2
//         }
//
//         fn is_valid_exchange_code(&self) -> bool {
//             self.user_number
//                 .chars()
//                 .nth(3)
//                 .unwrap()
//                 .to_digit(10)
//                 .unwrap()
//                 >= 2
//         }
//     }
// }
