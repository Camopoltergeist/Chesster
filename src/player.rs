pub enum Player {
    White,
    Black
}

impl Player {
    pub fn opposite(&self) -> Player {
        if let Self::White = self {
            return Self::Black;
        }
        else {
            return Self::White;
        }
    }
}