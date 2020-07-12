pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    let mut values: Vec<String> = vec![];
    let mut operations: Vec<String> = vec![];

    let mut iterations_to_skip = 0;
    let mut prev_char = 'W';

    for (i, c) in command.chars().enumerate() {
        if iterations_to_skip > 0 {
            iterations_to_skip -= 1;
            continue;
        }
        if prev_char == ' ' {
            if c == 'p' || c == 'm' || c == 'd' || c == 'r' {
                let operation = command
                    .get(i..)
                    .unwrap()
                    .chars()
                    .take_while(|c| *c != ' ')
                    .collect::<String>();

                if &operation == "plus"
                    || &operation == "minus"
                    || &operation == "multiplied"
                    || &operation == "divided"
                    || &operation == "raised"
                {
                    if values.len() == 0 || values.len() == operations.len() {
                        return None;
                    }
                    iterations_to_skip = operation.len() - 1;
                    operations.push(operation);
                }
            }
            if c.to_digit(10).is_some() || c == '-' {
                let number = command
                    .get(i..)
                    .unwrap()
                    .chars()
                    .take_while(|c| !c.to_digit(10).is_none() || *c == '-')
                    .collect::<String>();

                if values.len() > operations.len() {
                    return None;
                }

                if (command.len() - 1 - 1) == i && operations.len() == 0 {
                    return Some(number.parse().unwrap());
                }
                iterations_to_skip = number.len() - 1;
                values.push(number);
            }
        }

        prev_char = c;
    }

    if operations.len() == 0 || values.len() == 0 {
        return None;
    }
    let mut result: i32 = 0;

    for (i, value) in values.iter().enumerate() {
        if result == 0 {
            result = value.parse::<i32>().unwrap();
        } else {
            match operations.get(i - 1).unwrap().as_str() {
                "plus" => result += value.parse::<i32>().unwrap(),
                "minus" => result -= value.parse::<i32>().unwrap(),
                "multiplied" => result *= value.parse::<i32>().unwrap(),
                "divided" => result /= value.parse::<i32>().unwrap(),
                "raised" => result = result.pow(value.parse::<u32>().unwrap()),
                _ => {}
            }
        }
    }

    Some(result)
}
