pub fn encrypt(input: &str) -> String {
    let mut normalized_input = normalize(input.to_owned());
    let length = normalized_input.len();
    let dimension = Dimension::new(length);
    add_empty_spaces(&mut normalized_input, dimension.delta());

    let mut result = String::new();
    for i in 0..dimension.columns {
        for j in 0..dimension.rows {
            result.push(
                normalized_input
                    .get(i + (j * dimension.columns))
                    .unwrap()
                    .to_owned(),
            );
        }

        if i < dimension.columns - 1 {
            result.push(' ');
        }
    }

    result
}

fn add_empty_spaces(input: &mut Vec<char>, count: usize) {
    for _ in 0..count {
        input.push(' ');
    }
}

fn normalize(input: String) -> Vec<char> {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .collect::<String>()
        .chars()
        .collect()
}

#[derive(Debug)]
struct Dimension {
    rows: usize,
    columns: usize,
    length: usize,
}

impl Dimension {
    fn new(length: usize) -> Self {
        let mut rows = 0;
        let mut columns = 0;
        loop {
            if rows * columns >= length {
                break;
            }
            columns += 1;
            if rows * columns >= length {
                break;
            }
            rows += 1;
        }

        Dimension {
            rows,
            columns,
            length,
        }
    }

    fn product(&self) -> usize {
        self.rows * self.columns
    }

    fn delta(&self) -> usize {
        self.product() - self.length
    }
}
