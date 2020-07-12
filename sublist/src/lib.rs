#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    match first_list.len() > second_list.len() {
        true => {
            if second_list.is_empty() {
                return Comparison::Superlist;
            }

            let mut window = first_list.windows(second_list.len());
            while let Some(w) = window.next() {
                if second_list == w {
                    return Comparison::Superlist;
                }
            }
        }
        false => {
            if first_list.is_empty() {
                return Comparison::Sublist;
            }

            let mut window = second_list.windows(first_list.len());
            while let Some(w) = window.next() {
                if first_list == w {
                    return Comparison::Sublist;
                }
            }
        }
    };

    return Comparison::Unequal;
}
