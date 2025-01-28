#[derive(PartialEq, Eq, Clone, Copy)]
pub struct TilePosition {
    column: u32,
    rank: u32
}

impl TilePosition {
    pub const fn new(column: u32, rank: u32) -> Self {
        Self {
            column,
            rank
        }
    }

    pub fn column(&self) -> u32 {
        self.column
    }

    pub fn rank(&self) -> u32 {
        self.rank
    }

    pub const fn bit_offset(&self) -> u32 {
        self.column + self.rank * 8
    }

    pub fn from_bit_offset(bit_offset: u32) -> Self {
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
}

