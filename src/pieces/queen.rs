use crate::{board::{bitboard::Bitboard, move_mask::QUEEN_MASKS, tile_position::TilePosition}, piece::{Piece, PieceType}, player::Player};

pub struct Queen {
    player: Player,
    tile_position: TilePosition
}

impl Queen {
    pub fn new(player: Player, tile_position: TilePosition) -> Self {
        Self {
            player,
            tile_position
        }
    }

    pub const fn get_movement_mask(tile_position: TilePosition) -> Bitboard {
        QUEEN_MASKS[tile_position.bit_offset() as usize]
    }
}

impl Piece for Queen {
    fn piece_type(&self) -> PieceType {
        PieceType::Queen
    }

    fn player(&self) -> Player {
        self.player
    }

    fn tile_position(&self) -> TilePosition {
        self.tile_position
    }

    fn movement_mask(&self) -> Bitboard {
        Self::get_movement_mask(self.tile_position)
    }
}