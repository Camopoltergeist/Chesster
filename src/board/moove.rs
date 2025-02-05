use crate::{piece::PieceType, player::Player};

use super::tile_position::TilePosition;

#[derive(Debug)]
pub enum Move {
    Basic(BasicMove),
    Castling(CastlingMove),
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
            Self::Castling(castling_move) => {
                let side = match castling_move.side {
                    CastleSide::KingSide => "King Side",
                    CastleSide::QueenSide => "Queen Side"
                };

                return format!("Castling {}", side);
            }
            _ => unimplemented!()
        }
    }
}

impl From<BasicMove> for Move {
    fn from(value: BasicMove) -> Self {
        Self::Basic(value)
    }
}

impl From<CastlingMove> for Move {
    fn from(value: CastlingMove) -> Self {
        Self::Castling(value)
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

#[derive(Debug)]
pub struct CastlingMove {
    side: CastleSide
}

impl CastlingMove {
    pub fn new(side: CastleSide) -> Self {
        Self {
            side
        }
    }

    pub fn side(&self) -> CastleSide {
        self.side.clone()
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