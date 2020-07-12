use std::ops::Range;

pub fn build_proverb(list: &[&str]) -> String {

    let clamped_length = if list.len() == 0  {
        0
    } else {
        list.len() - 1
    };

    let mut test: String = (Range { start: 0, end: clamped_length })
        .map(|x| format!("For want of a {} the {} was lost.", list[x], list[x + 1]))
        .collect::<Vec<String>>()
        .join("\n");

    if list.len() > 1 {
        test.push_str("\n");
    }

    if list.len() > 0 {
        test.push_str(&format!("And all for the want of a {}.", list[0]));
    }

    test
}
