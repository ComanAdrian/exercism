use convertor::PigLatinSentenceConvertor;

pub fn translate(input: &str) -> String {
    let pig_latin_convertor: PigLatinSentenceConvertor = PigLatinSentenceConvertor::new(input);
    pig_latin_convertor.translate()
}

mod convertor {
    const VOWELS: [char; 8] = ['a', 'e', 'i', 'o', 'u', 'i', 'a', 'e'];

    pub struct PigLatinSentenceConvertor {
        sentence: String,
    }

    impl PigLatinSentenceConvertor {
        pub fn new(sentence: &str) -> Self {
            PigLatinSentenceConvertor {
                sentence: sentence.to_owned(),
            }
        }

        pub fn translate(&self) -> String {
            let separated_sentence: Vec<&str> = self.sentence.split(" ").collect();

            let result = separated_sentence
                .iter()
                .map(|word| PigLatinWordConvertor::new(word).convert_single_word())
                .collect::<Vec<String>>()
                .join(" ");
            result
        }
    }

    struct PigLatinWordConvertor {
        input: String,
    }

    impl<'a> PigLatinWordConvertor {
        const SUFFIX: &'a str = "ay";

        fn new(input: &str) -> Self {
            PigLatinWordConvertor {
                input: input.to_owned(),
            }
        }

        fn convert_single_word(&self) -> String {
            let mut result = self.input.to_owned();

            if self.should_append_prefix() {
                let mut delimiter_position = 1;
                let mut iter = self.input.chars().peekable();

                loop {
                    let char = iter.next().unwrap();
                    let next_peek = iter.peek().unwrap();

                    if self.contains_qu(&char, next_peek) {
                        delimiter_position += 1;
                        break;
                    }

                    if self.starts_with_y_followed_by_a_vowel() || self.is_a_vowel_or_y(next_peek) {
                        break;
                    }

                    delimiter_position += 1;
                }

                let (preffix_to_append, root) = self.input.split_at(delimiter_position);
                result = root.to_owned();
                result.push_str(preffix_to_append);
            }

            result.push_str(PigLatinWordConvertor::SUFFIX);

            result
        }

        fn should_append_prefix(&self) -> bool {
            !(self.starts_with_a_vowel()
                || self.starts_with_y_followed_by_a_cons()
                || self.starts_with_xr())
        }

        fn starts_with_a_vowel(&self) -> bool {
            VOWELS.contains(&self.input.chars().nth(0).unwrap())
        }

        fn starts_with_y_followed_by_a_cons(&self) -> bool {
            let mut iter = self.input.chars().peekable();

            iter.next().unwrap() == 'y' && !VOWELS.contains(&iter.peek().unwrap())
        }

        fn starts_with_xr(&self) -> bool {
            self.input.split_at(2).0 == "xr"
        }

        fn starts_with_y_followed_by_a_vowel(&self) -> bool {
            let mut iter = self.input.chars().peekable();

            iter.next().unwrap() == 'y' && VOWELS.contains(&iter.peek().unwrap())
        }

        fn is_a_vowel_or_y(&self, c: &char) -> bool {
            VOWELS.contains(c) || *c == 'y'
        }

        fn contains_qu(&self, char: &char, next_char: &char) -> bool {
            *char == 'q' && *next_char == 'u'
        }
    }
}
