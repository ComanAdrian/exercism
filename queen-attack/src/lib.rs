#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !ChessPosition::is_valid(rank) || !ChessPosition::is_valid(file) {
            return None;
        }
        Some(ChessPosition(rank, file))
    }

    fn is_valid(index: i32) -> bool {
        -1 < index && index < 8
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.0 == other.position.0 || self.position.1 == other.position.1 {
            return true;
        }

        if (self.position.0 < other.position.0) == (self.position.1 < other.position.1) {
            return self.position.1 - (self.position.0 - other.position.0) == other.position.1;
        } else {
            return self.position.1 + (self.position.0 - other.position.0) == other.position.1;
        }
    }
}
