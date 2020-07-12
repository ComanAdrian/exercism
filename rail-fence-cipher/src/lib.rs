pub struct RailFence {
    rails: usize,
}

#[derive(Debug)]
pub enum Direction {
    Downside,
    Upside,
}

impl Direction {
    fn swap(self) -> Direction {
        match self {
            Direction::Upside => Direction::Downside,
            Direction::Downside => Direction::Upside,
        }
    }
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut cypher: Vec<Vec<char>> = vec![];
        for _ in 0..self.rails {
            cypher.push(vec![])
        }

        let mut current_rail_index: usize = 0;
        let mut direction: Direction = Direction::Downside;
        for c in text.chars() {
            cypher.get_mut(current_rail_index).unwrap().push(c);

            match direction {
                Direction::Downside => current_rail_index += 1,
                Direction::Upside => current_rail_index -= 1,
            }

            if self.should_change_direction(current_rail_index) {
                direction = direction.swap();
            }
        }

        cypher
            .iter()
            .map(|v| v.iter().collect::<String>())
            .collect::<String>()
    }

    fn should_change_direction(&self, current_rail_index: usize) -> bool {
        current_rail_index == 0 || current_rail_index == self.rails - 1
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut text: Vec<char> = vec![' '; cipher.len()];
        let mut current_rail_index = 0;
        let mut index_to_insert_char = current_rail_index;
        let mut skip_points: usize = self.skip_points_downside(current_rail_index + 1);

        let mut direction: Direction = Direction::Downside;

        for c in cipher.chars() {
            text[index_to_insert_char] = c;
            index_to_insert_char += skip_points;

            if self.should_recalculate_skip_points(current_rail_index) {
                match direction {
                    Direction::Downside => {
                        skip_points = self.skip_points_upside(current_rail_index + 1)
                    }
                    Direction::Upside => {
                        skip_points = self.skip_points_downside(current_rail_index + 1)
                    }
                }
                direction = direction.swap();
            }

            if index_to_insert_char >= cipher.len() {
                current_rail_index += 1;
                index_to_insert_char = current_rail_index;
                skip_points = self.skip_points_downside(current_rail_index + 1);
            }
        }

        text.iter().collect::<String>()
    }

    fn should_recalculate_skip_points(&self, current_rail_index: usize) -> bool {
        current_rail_index != 0 || current_rail_index == self.rails + 1
    }

    fn skip_points_downside(&self, current_rail: usize) -> usize {
        let adjusted_current_rail = if (current_rail) >= self.rails {
            // when current_rail is the last one, skip_points will be the same as for the first rail
            1
        } else {
            current_rail
        };
        let loop_iteration = 1;
        let bottom_rail = 1;
        let middle_rails_below = self.rails - adjusted_current_rail - bottom_rail;

        1 + middle_rails_below * 2 + loop_iteration
    }

    fn skip_points_upside(&self, current_rail: usize) -> usize {
        let loop_iteration = 1;
        let top_rail = 1;
        let rails_above = current_rail - 1;
        let middle_rails_above = rails_above - top_rail;

        1 + middle_rails_above * 2 + loop_iteration
    }
}
