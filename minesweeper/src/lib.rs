pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = minefield
        .into_iter()
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for row in minefield.into_iter().enumerate() {
        for (i, c) in row.1.chars().enumerate() {
            if c == '*' {
                increment_minefield_boxes(&mut result, (row.0, i));
            }
        }
    }

    result
        .iter()
        .map(|v| v.iter().collect::<String>())
        .collect()
}

fn increment_minefield_boxes(minefield: &mut Vec<Vec<char>>, coordinate: (usize, usize)) {
    let bounds = Bounds::new(
        coordinate,
        (minefield.len() - 1, minefield.get(0).unwrap().len() - 1),
    );

    for i in bounds.left..bounds.right + 1 {
        for j in bounds.up..bounds.bottom + 1 {
            if minefield[i][j] == '*' {
                continue;
            } else {
                let char = minefield[i][j];
                if let Ok(x) = char.to_string().parse::<u32>() {
                    minefield[i][j] = std::char::from_digit(x + 1, 10).unwrap();
                } else {
                    minefield[i][j] = std::char::from_digit(1, 10).unwrap();
                };
            }
        }
    }
}

struct Bounds {
    left: usize,
    right: usize,
    up: usize,
    bottom: usize,
}

impl Bounds {
    pub fn new(coordinate: (usize, usize), dimension: (usize, usize)) -> Bounds {
        Bounds {
            left: Bounds::min(coordinate.0),
            right: Bounds::max(coordinate.0, dimension.0),
            up: Bounds::min(coordinate.1),
            bottom: Bounds::max(coordinate.1, dimension.1),
        }
    }

    fn min(begin: usize) -> usize {
        if begin == 0 {
            0
        } else {
            begin - 1
        }
    }

    fn max(end: usize, max: usize) -> usize {
        if end == max {
            max
        } else {
            end + 1
        }
    }
}
