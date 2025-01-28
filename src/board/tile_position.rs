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
}

