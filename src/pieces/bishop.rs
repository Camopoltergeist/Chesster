use crate::{board::{bitboard::Bitboard, move_mask::BISHOP_MASKS, tile_position::TilePosition}, piece::{Piece, PieceType}, player::Player};

use const_for::const_for;

pub struct Bishop {
    player: Player,
    tile_position: TilePosition,
}

impl Bishop {
    pub fn new(player: Player, tile_position: TilePosition) -> Self {
        Self {
            player,
            tile_position
        }
    }

    pub const fn get_movement_mask(tile_position: TilePosition) -> Bitboard {
        BISHOP_MASKS[tile_position.bit_offset() as usize]
    }

    pub const fn generate_movement_mask(tile_position: TilePosition) -> Bitboard {
        let rank_mask = Bitboard::get_diagonal_mask_asc(tile_position.column(), tile_position.rank());
        let column_mask = Bitboard::get_diagonal_mask_des(tile_position.column(), tile_position.rank());

        Bitboard(rank_mask.0 ^ column_mask.0)
    }

    pub const fn generate_all_movement_masks() -> [Bitboard; 64] {
        let mut masks = [Bitboard(0); 64];

        const_for!(rank in 0..8 => {
            const_for!(column in 0..8 => {
                let tile_pos = TilePosition::new(column, rank);

                let mask = Bishop::generate_movement_mask(tile_pos);

                masks[tile_pos.bit_offset() as usize] = mask;
            })
        });

        masks
    }
}

impl Piece for Bishop {
    fn piece_type(&self) -> PieceType {
        PieceType::Bishop
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