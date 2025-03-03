use crate::board::{moove::Move, position::Position};

pub struct Match {
    position: Position
}

impl Match {
    pub fn new() -> Self {
        Self {
            position: Position::default()
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn set_position(&mut self, position: &Position) {
        self.position = position.clone();
    }

    pub fn make_move(&mut self, moove: Move) {
        self.position.make_move(moove);
    }
}