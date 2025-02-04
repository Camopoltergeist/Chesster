#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Player {
    White,
    Black
}

impl Player {
    pub const fn opposite(&self) -> Player {
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

    pub fn from_fen_piece_char(c: char) -> Self {
        if c.is_ascii_lowercase() {
            return Self::Black;
        }
        
        return Self::White;
    }

    pub fn from_fen_char(c: char) -> Self {
        if c == 'w' {
            return Self::White;
        }

        return Self::Black;
    }

    pub fn castling_target_rank(&self) -> u32 {
        match self {
            Player::White => 0,
            Player::Black => 7
        }
    }
}