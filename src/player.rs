#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::White => "White",
            Self::Black => "Black"
        }
    }
}