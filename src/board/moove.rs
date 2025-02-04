use crate::{piece::PieceType, player::Player};

use super::tile_position::TilePosition;

#[derive(Debug)]
pub struct Move {
    from: TilePosition,
    to: TilePosition,
    castling: bool,
    promotion: Option<PieceType>
}

impl Move {
    pub fn new(from: TilePosition, to: TilePosition) -> Self {
        Self {
            from,
            to,
            castling: false,
            promotion: None
        }
    }

    pub fn with_castling(player: Player, king_side: bool) -> Self {
        let rank = match player {
            Player::White => 0,
            Player::Black => 7
        };

        let from_column = 4;

        let to_column = if king_side { 6 } else { 2 };

        let from = TilePosition::new(from_column, rank);
        let to = TilePosition::new(to_column, rank);

        Self {
            from,
            to,
            castling: true,
            promotion: None
        }
    }

    pub fn from(&self) -> TilePosition {
        self.from
    }

    pub fn to(&self) -> TilePosition {
        self.to
    }
}