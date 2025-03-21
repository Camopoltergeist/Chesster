//! Implementations for basic, castling, en passant and promoting moves.
//! The name is "moove" as `move` is a keyword in Rust.

use crate::{player::Player, player_piece::PlayerPiece};

use super::tile_position::TilePosition;

/// Enum representing any possible chess moves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Move {
    Basic(BasicMove),
    Castling(CastlingMove),
    EnPassant(EnPassantMove),
    Promoting(PromotingMove)
}

impl Move {
    /// Quickly construct [`Self::Basic`] variant.
    /// 
    /// Not recommended to be used outside of testing as it is slow.
    /// 
    /// Both `from` and `to` parameters are chess tiles as `&str`.
    pub fn debug_new_basic(from: &str, to: &str) -> Move {
        Move::Basic(BasicMove::new(TilePosition::from_tile_str(from).unwrap(), TilePosition::from_tile_str(to).unwrap()))
    }

    /// Construct [`Self::Castling`] variant for specified [`player`][Player] and [`side`][CastleSide].
    pub fn new_castling(player: Player, side: CastleSide) -> Move {
        Move::Castling(CastlingMove::new(player, side))
    }

    /// Quickly construct new [`Self::EnPassant`] variant.
    /// 
    /// Not recommended to be used outside of testing as it is slow.
    /// 
    /// All `from`, `to` and `target_square` parameters are chess tiles as `&str`.  
    /// Validity of `target_square` is not checked.
    pub fn debug_new_en_passant(from: &str, to: &str, target_square: &str) -> Move {
        Move::EnPassant(EnPassantMove::new(TilePosition::from_tile_str(from).unwrap(), TilePosition::from_tile_str(to).unwrap(), TilePosition::from_tile_str(target_square).unwrap()))
    }

    /// Quickly construct new [`Self::Promoting`] variant.
    /// 
    /// Not recommended to be used outside of testing as it is slow.
    /// 
    /// `from` and `to` parameters are chess tiles as `&str`.  
    /// [`promotion_piece`][PlayerPiece] is the piece this move promotes to. 
    pub fn debug_new_promoting(from: &str, to: &str, promotion_piece: PlayerPiece) -> Move {
        Move::Promoting(PromotingMove::new(TilePosition::from_tile_str(from).unwrap(), TilePosition::from_tile_str(to).unwrap(), promotion_piece))
    }

    /// Get target [`TilePosition`] of the king piece for specified [`player`][Player] and [`side`][CastleSide].
    pub fn get_castling_target(player: Player, side: CastleSide) -> TilePosition {
        let rank = player.castling_rank();
        let column = side.castling_king_target_column();

        TilePosition::new(column, rank)
    }

    /// Get debug [`String`] of current position.
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
            },
            Self::EnPassant(en_passant_move) => {
                return format!("{} -> {}", en_passant_move.from.notation_string(), en_passant_move.to.notation_string());
            },
            Self::Promoting(promoting_move) => {
                return format!("{} -> {} {:?}", promoting_move.from.notation_string(), promoting_move.to.notation_string(), promoting_move.promotion_piece.piece());
            }
        }
    }

    /// Get [`TilePosition`] of the move's starting position.
    pub fn from_position(&self) -> TilePosition {
        match self {
            Self::Basic(basic_move) => basic_move.from_position(),
            Self::Castling(castling_move) => castling_move.from_position(),
            Self::EnPassant(en_passant_move) => en_passant_move.from_position(),
            Self::Promoting(promoting_move) => promoting_move.from_position(),
        }
    }

    /// Get [`TilePosition`] of the move's landing position.
    pub fn to_position(&self) -> TilePosition {
        match self {
            Self::Basic(basic_move) => basic_move.to_position(),
            Self::Castling(castling_move) => castling_move.to_position(),
            Self::EnPassant(en_passant_move) => en_passant_move.to_position(),
            Self::Promoting(promoting_move) => promoting_move.to_position(),
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

impl From<EnPassantMove> for Move {
    fn from(value: EnPassantMove) -> Self {
        Self::EnPassant(value)
    }
}

impl From<PromotingMove> for Move {
    fn from(value: PromotingMove) -> Self {
        Self::Promoting(value)
    }
}

/// Represents basic chess moves including captures.
#[derive(Debug, Clone, PartialEq, Eq)]
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

    /// See [`Move::from_position`].
    pub fn from_position(&self) -> TilePosition {
        self.from
    }

    /// See [`Move::to_position`].
    pub fn to_position(&self) -> TilePosition {
        self.to
    }
}

impl From<EnPassantMove> for BasicMove {
    fn from(value: EnPassantMove) -> Self {
        Self::new(value.from, value.to)
    }
}

impl From<PromotingMove> for BasicMove {
    fn from(value: PromotingMove) -> Self {
        Self::new(value.from, value.to)
    }
}

/// Represents castling chess move.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastlingMove {
    player: Player,
    side: CastleSide,
}

impl CastlingMove {
    pub fn new(player: Player, side: CastleSide) -> Self {
        Self {
            player,
            side
        }
    }

    /// See [`Move::from_position`].
    pub fn from_position(&self) -> TilePosition {
        self.player.castling_king_starting_position()
    }

    /// See [`Move::to_position`].
    pub fn to_position(&self) -> TilePosition {
        let column = self.side.castling_king_target_column();
        let rank = self.player.castling_rank();

        TilePosition::new(column, rank)
    }

    /// Starting [`TilePosition`] of the rook
    pub fn rook_from_position(&self) -> TilePosition {
        let column = self.side.castling_rook_starting_column();
        let rank = self.player.castling_rank();

        TilePosition::new(column, rank)
    }

    /// Landing [`TilePosition`] of the rook
    pub fn rook_to_position(&self) -> TilePosition {
        let column = self.side.castling_rook_target_column();
        let rank = self.player.castling_rank();

        TilePosition::new(column, rank)
    }

    /// Get the king's movement as [`BasicMove`]
    pub fn king_basic_move(&self) -> BasicMove {
        BasicMove::new(self.from_position(), self.to_position())
    }

    /// Get the rook's movement as [`BasicMove`]
    pub fn rook_basic_move(&self) -> BasicMove {
        BasicMove::new(self.rook_from_position(), self.rook_to_position())
    }

    /// [`Player`] that made the move
    pub fn player(&self) -> Player {
        self.player
    }

    /// [`CastleSide`] of the move
    pub fn side(&self) -> CastleSide {
        self.side.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CastleSide {
    KingSide,
    QueenSide
}

impl CastleSide {
    pub fn castling_king_target_column(&self) -> u32 {
        match self {
            Self::KingSide => 6,
            Self::QueenSide => 2
        }
    }

    pub fn castling_rook_starting_column(&self) -> u32 {
        match self {
            Self::KingSide => 7,
            Self::QueenSide => 0,
        }
    }

    pub fn castling_rook_target_column(&self) -> u32 {
        match self {
            Self::KingSide => 5,
            Self::QueenSide => 3
        }
    }
}

/// Represents en passant chess move.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnPassantMove {
    from: TilePosition,
    to: TilePosition,
    captured_tile: TilePosition,
}

impl EnPassantMove {
    pub fn new(from: TilePosition, to: TilePosition, captured_tile: TilePosition) -> Self {
        Self {
            from,
            to,
            captured_tile
        }
    }

    /// See [`Move::from_position`].
    pub fn from_position(&self) -> TilePosition {
        self.from
    }

    /// See [`Move::to_position`].
    pub fn to_position(&self) -> TilePosition {
        self.to
    }

    /// Tile which is the target of en passant capture.
    pub fn captured_tile(&self) -> TilePosition {
        self.captured_tile
    }
}

/// Represents a promoting chess move.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromotingMove {
    from: TilePosition,
    to: TilePosition,
    promotion_piece: PlayerPiece
}

impl PromotingMove {
    pub fn new(from: TilePosition, to: TilePosition, promotion_piece: PlayerPiece) -> Self {
        Self {
            from,
            to,
            promotion_piece
        }
    }

    /// See [`Move::from_position`].
    pub fn from_position(&self) -> TilePosition {
        self.from
    }

    /// See [`Move::to_position`].
    pub fn to_position(&self) -> TilePosition {
        self.to
    }

    /// [`PlayerPiece`] which this move promotes to.
    pub fn promotion_piece(&self) -> PlayerPiece {
        self.promotion_piece
    }
}