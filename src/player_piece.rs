use crate::{piece::PieceType, player::Player};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerPiece {
    player: Player,
    piece: PieceType
}

impl PlayerPiece {
    pub fn new(player: Player, piece: PieceType) -> Self {
        Self {
            player,
            piece
        }
    }

    pub fn player(&self) -> Player {
        self.player
    }

    pub fn piece(&self) -> PieceType {
        self.piece
    }
}