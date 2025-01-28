use crate::{piece::Piece, player::Player};

#[derive(Debug)]
pub struct PlayerPiece {
    player: Player,
    piece: Piece
}

impl PlayerPiece {
    pub fn player(&self) -> Player {
        self.player
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }
}