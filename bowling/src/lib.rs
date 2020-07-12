#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    current_score: u16,
    current_frame: u16,
    current_frame_roll: FrameRoll,
    current_frame_score: u16,
    bonus_rolls: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            current_score: 0,
            current_frame: 0,
            current_frame_score: 0,
            current_frame_roll: FrameRoll::First,
            bonus_rolls: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.not_enough_pins_left(pins) {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.is_fill_frame() {
            return self.count_fill_frame_bonus(pins);
        }

        if self.is_last_frame() {
            return Err(Error::GameComplete);
        }

        if self.is_first_frame_roll() {
            self.count_bonus(pins);

            if self.is_strike(pins) {
                self.bonus_rolls += 2;
                self.set_next_frame();
            } else {
                self.current_frame_score = pins;
                self.current_frame_roll = FrameRoll::Second;
            }

            self.current_score += pins;
        } else if self.is_second_frames_roll() {
            self.count_bonus(pins);

            self.handle_spare_roll(pins);

            self.current_score += pins;

            self.set_next_frame()
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.current_frame < 10 || self.bonus_rolls != 0 {
            return None;
        }
        Some(self.current_score)
    }

    fn not_enough_pins_left(&self, pins: u16) -> bool {
        if (pins > 10) || (self.current_frame_score + pins > 10) {
            return true;
        } else {
            return false;
        }
    }

    fn is_fill_frame(&self) -> bool {
        self.is_last_frame() && self.bonus_rolls > 0
    }

    fn is_last_frame(&self) -> bool {
        self.current_frame == 10
    }

    fn is_first_frame_roll(&self) -> bool {
        self.current_frame_roll == FrameRoll::First
    }

    fn is_second_frames_roll(&self) -> bool {
        self.current_frame_roll == FrameRoll::Second
    }

    fn is_strike(&self, pins: u16) -> bool {
        pins == 10
    }

    fn handle_spare_roll(&mut self, pins: u16) -> () {
        if self.current_frame_score + pins == 10 {
            self.bonus_rolls += 1;
        }
    }

    fn set_next_frame(&mut self) -> () {
        self.current_frame += 1;
        self.current_frame_roll = FrameRoll::First;
        self.current_frame_score = 0;
    }

    fn count_fill_frame_bonus(&mut self, pins: u16) -> Result<(), Error> {
        if pins != 10 {
            self.current_frame_score += pins;
        }
        self.count_bonus(pins);
        return Ok(());
    }

    fn count_bonus(&mut self, pins: u16) -> () {
        if self.is_roll_preceded_by_two_strikes() {
            self.current_score += pins * 2;
            self.bonus_rolls -= 2;
        } else if self.is_roll_preceded_by_a_strike_or_spare() {
            self.current_score += pins;
            self.bonus_rolls -= 1;
        }
    }

    fn is_roll_preceded_by_two_strikes(&self) -> bool {
        self.bonus_rolls == 3
    }

    fn is_roll_preceded_by_a_strike_or_spare(&self) -> bool {
        self.bonus_rolls > 0
    }
}

#[derive(PartialEq)]
enum FrameRoll {
    First,
    Second
}