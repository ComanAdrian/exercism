pub fn verse(n: u32) -> String {
    let mut result: String = format!(
        "{bottle_counter_text} of beer on the wall, {bottle_counter_text} of beer.\n\
         {second_row_first_part}, {bottle_counter_text_decremented} of beer on the wall.\n",
        bottle_counter_text = get_bottle_counter_text(n as i32),
        second_row_first_part = get_second_row_first_part(n as i32),
        bottle_counter_text_decremented = get_bottle_counter_text((n as i32) - 1)
    );
    make_ascii_titlecase(&mut result);
    result
}

fn get_second_row_first_part(n: i32) -> String {
    if n > 0 {
        format!(
            "Take {bottle_enumeration_text} down and pass it around",
            bottle_enumeration_text = get_bottle_enumeration_text(n),
        )
    } else {
        "Go to the store and buy some more".to_owned()
    }
}
fn get_bottle_counter_text(n: i32) -> String {
    if n == 1 {
        format!("{} bottle", n)
    } else if n == 0 {
        format!("no more bottles")
    }
    else if n < 0 {
        format!("99 bottles")
    }
    else {
        format!("{} bottles", n)
    }
}

fn get_bottle_enumeration_text(n: i32) -> String {
    if n > 1 {
        format!("one")
    } else {
        format!("it")
    }
}

fn make_ascii_titlecase(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let song = (std::ops::Range { start: end, end: start + 1})
        .map(|n| {
            if n == start {
                return verse(n)
            }
            format!("\n{}", verse(n))
        })
        .rev()
        .collect::<Vec<String>>()
        .join("");

    song
}
