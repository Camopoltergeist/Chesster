use crate::{piece::PieceType, player::Player};

use super::tile_position::TilePosition;

#[derive(Debug)]
pub struct Move {
    from: TilePosition,
    to: TilePosition,
    castling: Option<CastleSide>,
    promotion: Option<PieceType>
}

impl Move {
    pub fn new(from: TilePosition, to: TilePosition) -> Self {
        Self {
            from,
            to,
            castling: None,
            promotion: None
        }
    }

    pub fn with_castling(player: Player, side: CastleSide) -> Self {
        let rank = match player {
            Player::White => 0,
            Player::Black => 7
        };

        let from_column = 4;

        let to_column = side.castling_target_column();

        let from = TilePosition::new(from_column, rank);
        let to = TilePosition::new(to_column, rank);

        Self {
            from,
            to,
            castling: Some(side),
            promotion: None
        }
    }

    pub fn from(&self) -> TilePosition {
        self.from
    }

    pub fn to(&self) -> TilePosition {
        self.to
    }

    pub fn get_castling_target(player: Player, side: CastleSide) -> TilePosition {
        let rank = player.castling_target_rank();
        let column = side.castling_target_column();

        TilePosition::new(column, rank)
    }
}

#[derive(Debug)]
pub enum CastleSide {
    KingSide,
    QueenSide
}

impl CastleSide {
    pub fn castling_target_column(&self) -> u32 {
        match self {
            Self::KingSide => 6,
            Self::QueenSide => 2
        }
    }
}