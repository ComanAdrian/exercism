const BRACKETS: [char; 3] = ['(', '[', '{'];
const CLOSING_BRACKETS: [char; 3] = [')', ']', '}'];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut open_brackets: Vec<char> = vec![];

    for c in string.chars() {
        if CLOSING_BRACKETS.contains(&c) {
            let index = CLOSING_BRACKETS
                .iter()
                .position(|&c_b| c_b == c)
                .unwrap();
            if open_brackets.last().unwrap() == &BRACKETS[index] {
                open_brackets.pop();
                continue;
            } else {
                return false;
            }
        }

        if BRACKETS.contains(&c) {
            open_brackets.push(c);
        }
    }

    open_brackets.len() == 0
}
