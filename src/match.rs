use crate::board::position::Position;

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
}