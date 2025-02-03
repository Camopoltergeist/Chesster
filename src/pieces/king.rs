use crate::{board::{bitboard::Bitboard, tile_position::TilePosition}, piece::{Piece, PieceType}, player::Player};

use const_for::const_for;

pub struct King {
    player: Player,
    tile_position: TilePosition
}

impl King {
    pub const MOVEMENT_MASKS: [Bitboard; 64] = Self::generate_all_movement_masks();

    pub fn new(player: Player, tile_position: TilePosition) -> Self {
        Self {
            player,
            tile_position
        }
    }

    pub const fn get_movement_mask(tile_position: TilePosition) -> Bitboard {
        Self::MOVEMENT_MASKS[tile_position.bit_offset() as usize]
    }

    pub const fn generate_movement_mask(tile_position: TilePosition) -> Bitboard {
        Bitboard::get_king_mask(tile_position.column(), tile_position.rank())
    }

    pub const fn generate_all_movement_masks() -> [Bitboard; 64] {
        let mut masks = [Bitboard(0); 64];

        const_for!(rank in 0..8 => {
            const_for!(column in 0..8 => {
                let tile_pos = TilePosition::new(column, rank);

                masks[tile_pos.bit_offset() as usize] = Self::generate_movement_mask(tile_pos);
            });
        });

        masks
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