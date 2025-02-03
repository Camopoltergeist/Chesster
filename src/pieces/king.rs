use crate::{board::{bitboard::Bitboard, move_mask::KING_MASKS, tile_position::TilePosition}, piece::{Piece, PieceType}, player::Player};

pub struct King {
    player: Player,
    tile_position: TilePosition
}

impl King {
    pub fn new(player: Player, tile_position: TilePosition) -> Self {
        Self {
            player,
            tile_position
        }
    }

    pub const fn get_movement_mask(tile_position: TilePosition) -> Bitboard {
        KING_MASKS[tile_position.bit_offset() as usize]
    }

    pub const fn generate_movement_mask(tile_position: TilePosition) -> Bitboard {
        Bitboard::get_king_mask(tile_position.column(), tile_position.rank())
    }
}

impl Piece for King {
    fn piece_type(&self) -> PieceType {
        PieceType::King
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