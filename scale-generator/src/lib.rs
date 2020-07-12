use crate::ScaleType::{Flat, Sharp};

static SHARPS: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
static FLATS: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
];

// TODO: implement error type
// pub enum ParseError {
//     InvalidTonic,
//     InvalidInterval,
// }

pub type Error = ();

pub enum ScaleType {
    Sharp,
    Flat,
}

pub struct Scale {
    scale_type: ScaleType,
    start_index: usize,
    intervals: Vec<u32>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let (start_index, scale_type) = match tonic {
            "C" => (0, Sharp),
            "c" => (0, Flat),
            "c#" => (1, Sharp),
            "Db" => (1, Flat),
            "D" => (2, Sharp),
            "d" => (2, Flat),
            "d#" => (3, Sharp),
            "Eb" | "eb" => (3, Flat),
            "E" | "e" => (4, Sharp),
            "F" | "f" => (5, Flat),
            "F#" | "f#" => (6, Sharp),
            "Gb" => (6, Flat),
            "G" => (7, Sharp),
            "g" => (7, Flat),
            "g#" => (8, Sharp),
            "Ab" => (8, Flat),
            "A" | "a" => (9, Sharp),
            "Bb" | "bb" => (10, Flat),
            "B" | "b" => (11, Sharp),
            _ => unreachable!(),
        };

        Ok(Scale {
            scale_type,
            start_index,
            intervals: intervals
                .chars()
                .map(|c| match c {
                    'm' => 1,
                    'M' => 2,
                    'A' => 3,
                    _ => unreachable!(),
                })
                .collect(),
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        let scale = match self.scale_type {
            Sharp => SHARPS,
            Flat => FLATS,
        };
        let mut result: Vec<String> = vec![];
        let mut current_index: usize = self.start_index;

        result.push(scale[self.start_index].to_string());
        for value in self
            .intervals
            .iter()
            .enumerate()
            .take_while(|(i, _)| *i != self.intervals.len() - 1)
            .map(|(_, &v)| v as usize)
        {
            current_index += value;
            current_index %= scale.len();
            result.push(scale[current_index].to_string())
        }

        result
    }
}
