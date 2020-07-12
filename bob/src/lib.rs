pub fn reply(message: &str) -> &str {
    let trimmed_message = message.clone().trim();
    let is_empty = trimmed_message.is_empty();

    if is_empty {
        return "Fine. Be that way!";
    }

    let is_simple_question = trimmed_message.chars().last().unwrap() == '?';

    let mut is_shouting_iterator = trimmed_message.chars().filter(|&x| x.is_ascii_alphabetic());
    let is_shouting = is_shouting_iterator.clone().count() > 0
        && is_shouting_iterator.all(|x| x.is_ascii_uppercase());

    if is_shouting && is_simple_question {
        return "Calm down, I know what I'm doing!";
    } else if is_simple_question {
        return "Sure.";
    } else if is_shouting {
        return "Whoa, chill out!";
    } else {
        return "Whatever.";
    }
}
