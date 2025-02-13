use crate::player::Player;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct TilePosition {
    column: u32,
    rank: u32
}

impl TilePosition {
    pub const fn new(column: u32, rank: u32) -> Self {
        debug_assert!(column <= 7);
        debug_assert!(rank <= 7);

        Self {
            column,
            rank
        }
    }

    pub fn notation_string(&self) -> String {
        let column_char = match self.column {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            _ => panic!("invalid column")
        };

        let rank_char = match self.rank {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => panic!("invalid rank")
        };

        format!("{}{}", column_char, rank_char)
    }

    pub const fn column(&self) -> u32 {
        self.column
    }

    pub const fn rank(&self) -> u32 {
        self.rank
    }

    pub const fn bit_offset(&self) -> u32 {
        self.column + self.rank * 8
    }

    pub const fn from_bit_offset(bit_offset: u32) -> Self {
        let column = bit_offset % 8;
        let rank = bit_offset / 8;

        return Self::new(column, rank);
    }

    /// Converts a tile &str to coordinates. For example b1 => (1, 0)
    /// Used for debugging purposes only. This function is kinda slow.
    pub fn from_tile_str(tile_str: &str) -> Result<Self, ()> {
        if tile_str.len() != 2 {
            return Err(());
        };

        let tile = tile_str.to_ascii_lowercase();

        let column_char = tile.chars().nth(0).unwrap();
        let rank_char = tile.chars().nth(1).unwrap();

        let column = match column_char {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err(()),
        };

        let rank = match rank_char {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return Err(()),
        };

        Ok(Self::new(column, rank))
    }

    pub fn get_en_passant_left_capture(&self, player: Player) -> Option<Self> {
        if self.column < 1 {
            return None;
        };

        let rank = match player {
            Player::White => 4,
            Player::Black => 3
        };

        return Some(TilePosition::new(self.column - 1, rank));
    }

    pub fn get_en_passant_right_capture(&self, player: Player) -> Option<Self> {
        if self.column > 6 {
            return None;
        };

        let rank = match player {
            Player::White => 4,
            Player::Black => 3
        };

        return Some(TilePosition::new(self.column + 1, rank));
    }

    pub fn get_pawn_advance_position(&self, player: Player) -> Self {
        let new_rank = match player {
            Player::White => {
                debug_assert!(self.rank < 7);

                self.rank + 1
            },
            Player::Black => {
                debug_assert!(self.rank > 0);

                self.rank - 1
            }
        };

        Self::new(self.column, new_rank)
    }
}

