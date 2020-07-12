use std::ops::Add;

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    coordinates: Coordinates,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            coordinates: Coordinates(x, y),
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        Robot {
            coordinates: self.coordinates,
            direction: direction,
        }
    }

    pub fn turn_left(self) -> Self {
        let direction = match self.direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };

        Robot {
            coordinates: self.coordinates,
            direction: direction,
        }
    }

    pub fn advance(self) -> Self {
        let coordinates_diff = match self.direction {
            Direction::North => Coordinates(0, 1),
            Direction::East => Coordinates(1, 0),
            Direction::South => Coordinates(0, -1),
            Direction::West => Coordinates(-1, 0),
        };

        Robot {
            coordinates: self.coordinates + coordinates_diff,
            direction: self.direction,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;

        for c in instructions.chars() {
            robot = match c {
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                'A' => robot.advance(),
                _ => robot,
            };
        }

        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.coordinates.0, self.coordinates.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

struct Coordinates(i32, i32);

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Coordinates) -> Self {
        Coordinates(self.0 + rhs.0, self.1 + rhs.1)
    }
}
