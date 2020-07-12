struct Direction {
    pub initial_moves: u32,
    pub available_moves: u32,
    pub current_coordinate: (usize, usize),
}

impl Direction {
    pub fn new(moves: u32, coordinates: (usize, usize)) -> Direction {
        Direction {
            initial_moves: moves,
            available_moves: moves,
            current_coordinate: coordinates,
        }
    }

    pub fn can_continue(&self) -> bool {
        self.available_moves != 0
    }

    pub fn update_matrix(&self, matrix: &mut Vec<Vec<u32>>, value: u32) {
        matrix[self.current_coordinate.0][self.current_coordinate.1] = value;
    }

    pub fn decrement_available_moves(&mut self) {
        self.available_moves -= 1;
    }

    pub fn is_last_available_move(&self) -> bool {
        self.available_moves == 0
    }

    pub fn increment_row_coordinate(&mut self) {
        self.current_coordinate.0 += 1;
    }

    pub fn increment_column_coordinate(&mut self) {
        self.current_coordinate.1 += 1;
    }

    pub fn decrement_row_coordinate(&mut self) {
        self.current_coordinate.0 -= 1;
    }

    pub fn decrement_column_coordinate(&mut self) {
        self.current_coordinate.1 -= 1;
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut right_direction = Direction::new(size, (0, 0));
    let mut bottom_direction = Direction::new(0, (0, 0));
    let mut left_direction = Direction::new(0, (0, 0));
    let mut up_direction = Direction::new(0, (0, 0));

    let mut matrix = make_matrix(size);

    for i in 1..size.pow(2) + 1 {
        if right_direction.can_continue() {
            right_direction.update_matrix(&mut matrix, i);
            right_direction.decrement_available_moves();

            if right_direction.is_last_available_move() {
                bottom_direction = Direction::new(
                    right_direction.initial_moves - 1,
                    (
                        right_direction.current_coordinate.0 + 1,
                        right_direction.current_coordinate.1,
                    ),
                );
            } else {
                right_direction.increment_column_coordinate();
            }
            continue;
        }

        if bottom_direction.can_continue() {
            bottom_direction.update_matrix(&mut matrix, i);
            bottom_direction.decrement_available_moves();

            if bottom_direction.is_last_available_move() {
                left_direction = Direction::new(
                    bottom_direction.initial_moves,
                    (
                        bottom_direction.current_coordinate.0,
                        bottom_direction.current_coordinate.1 - 1,
                    ),
                );
            } else {
                bottom_direction.increment_row_coordinate();
            }
            continue;
        }

        if left_direction.can_continue() {
            left_direction.update_matrix(&mut matrix, i);
            left_direction.decrement_available_moves();

            if left_direction.is_last_available_move() {
                up_direction = Direction::new(
                    left_direction.initial_moves - 1,
                    (
                        left_direction.current_coordinate.0 - 1,
                        left_direction.current_coordinate.1,
                    ),
                );
            } else {
                left_direction.decrement_column_coordinate();
            }
            continue;
        }

        if up_direction.can_continue() {
            up_direction.update_matrix(&mut matrix, i);
            up_direction.decrement_available_moves();

            if up_direction.is_last_available_move() {
                right_direction = Direction::new(
                    up_direction.initial_moves,
                    (
                        up_direction.current_coordinate.0,
                        up_direction.current_coordinate.1 + 1,
                    ),
                );
            } else {
                up_direction.decrement_row_coordinate();
            }
            continue;
        }
    }

    matrix
}

fn make_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = vec![];

    for i in 0..size {
        result.push(vec![]);

        for _ in 0..size {
            result[i as usize].push(0);
        }
    }

    result
}
