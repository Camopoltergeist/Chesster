use super::tile_position::TilePosition;

pub struct Move {
    from: TilePosition,
    to: TilePosition
}

impl Move {
    pub fn new(from: TilePosition, to: TilePosition) -> Self {
        Self {
            from,
            to
        }
    }

    pub fn from(&self) -> TilePosition {
        self.from
    }

    pub fn to(&self) -> TilePosition {
        self.to
    }
}