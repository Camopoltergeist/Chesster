pub struct TilePosition {
    column: u32,
    rank: u32
}

impl TilePosition {
    pub fn new(column: u32, rank: u32) -> Self {
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
}

