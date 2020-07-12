use std::cell::RefCell;

use provider::LetterIndexProvider;

mod provider {
    use std::ops::Range;

    enum Direction {
        ToCenter,
        FromCenter,
    }

    pub struct LetterIndexProvider {
        pub first: usize,
        pub last: usize,
        middle_letter_index: usize,
        direction: Direction,
    }

    impl LetterIndexProvider {
        pub fn new(middle_letter_index: usize) -> LetterIndexProvider {
            LetterIndexProvider {
                first: middle_letter_index,
                last: middle_letter_index,
                middle_letter_index,
                direction: Direction::ToCenter,
            }
        }

        pub fn diamond_length(&self) -> usize {
            self.middle_letter_index * 2 + 1
        }

        pub fn letter_index_to_insert(&self, iteration: usize) -> usize {
            if iteration <= self.middle_letter_index {
                iteration
            } else {
                self.middle_letter_index * 2 - iteration
            }
        }

        pub fn notify_iteration_complete(&mut self) {
            self.update_letter_indexes();
            self.update_direction_if_needed();
        }

        pub fn is_middle_row(&self) -> bool {
            (self.last - self.first) != 0
        }

        pub fn first_gap_range(&self) -> Range<usize> {
            0..self.first
        }

        pub fn middle_gap_range(&self) -> Range<usize> {
            self.first..self.last - 1
        }

        pub fn last_gap_range(&self) -> Range<usize> {
            self.last..self.diamond_length() - 1
        }

        pub fn is_last_row(&self, iter: usize) -> bool {
            iter == self.diamond_length() - 1
        }

        fn update_direction_if_needed(&mut self) {
            if self.first == 0 {
                self.direction = Direction::FromCenter;
            }
        }

        fn update_letter_indexes(&mut self) {
            match self.direction {
                Direction::ToCenter => {
                    self.first -= 1;
                    self.last += 1;
                }
                Direction::FromCenter => {
                    self.first += 1;
                    self.last -= 1;
                }
            };
        }
    }
}

pub fn get_diamond(c: char) -> Vec<String> {
    let alphabet = create_alphabet();
    let middle_letter_index = alphabet.iter().position(|char| char == &c).unwrap();
    let result: RefCell<Vec<String>> = RefCell::new(vec![]);

    let mut letter_index_provider = LetterIndexProvider::new(middle_letter_index);
    for i in 0..letter_index_provider.diamond_length() {
        let letter_to_insert_index = letter_index_provider.letter_index_to_insert(i);
        if result.borrow().get(i).is_none() {
            result.borrow_mut().push(String::new());
        }

        let add_gap = |_| result.borrow_mut().get_mut(i).unwrap().push(' ');
        let letter_to_add = *alphabet.get(letter_to_insert_index).unwrap();

        letter_index_provider.first_gap_range().for_each(add_gap);

        result.borrow_mut().get_mut(i).unwrap().push(letter_to_add);

        if letter_index_provider.is_middle_row() {
            letter_index_provider.middle_gap_range().for_each(add_gap);

            result.borrow_mut().get_mut(i).unwrap().push(letter_to_add);
        }

        letter_index_provider.last_gap_range().for_each(add_gap);

        if letter_index_provider.is_last_row(i) {
            break;
        }

        letter_index_provider.notify_iteration_complete();
    }

    result.into_inner()
}

fn create_alphabet() -> Vec<char> {
    let alphabet_letters_count = 26;
    let mut alphabet: Vec<char> = vec![];
    let a_letter = 'A' as u32;

    for i in 0..alphabet_letters_count {
        alphabet.push(std::char::from_u32(a_letter + i).unwrap());
    }

    alphabet
}
