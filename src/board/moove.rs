use crate::{piece::PieceType, player::Player};

use super::tile_position::TilePosition;

#[derive(Debug)]
pub enum Move {
    Basic(BasicMove),
    Castling {
        castling: CastleSide,
    },
    Promoting {
        from: TilePosition,
        to: TilePosition,
        promotion: PieceType
    }
}

impl Move {
    pub fn get_castling_target(player: Player, side: CastleSide) -> TilePosition {
        let rank = player.castling_target_rank();
        let column = side.castling_target_column();

        TilePosition::new(column, rank)
    }

    pub fn debug_string(&self) -> String {
        match self {
            Self::Basic(basic_move) => {
                return format!("{} -> {}", basic_move.from.notation_string(), basic_move.to.notation_string());
            },
            _ => unimplemented!()
        }
    }
}

impl From<BasicMove> for Move {
    fn from(value: BasicMove) -> Self {
        Self::Basic(value)
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct BasicMove {
    from: TilePosition,
    to: TilePosition,
}

impl BasicMove {
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