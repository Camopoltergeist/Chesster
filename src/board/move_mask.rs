use crate::{board::tile_position::TilePosition, pieces::pawn::Pawn};

use super::bitboard::Bitboard;

use const_for::const_for;

pub const WHITE_PAWN_MASKS: [Bitboard; 64] = generate_white_pawn_masks();
pub const BLACK_PAWN_MASKS: [Bitboard; 64] = generate_black_pawn_masks();

pub const fn generate_white_pawn_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];

    const_for!(rank in 0..8 => {
        const_for!(column in 0..8 => {
            let tile_pos = TilePosition::new(column, rank);

            masks[tile_pos.bit_offset() as usize] = Pawn::generate_movement_mask(tile_pos, crate::player::Player::White);
        });
    });

    masks
}

pub const fn generate_black_pawn_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];

    const_for!(rank in 0..8 => {
        const_for!(column in 0..8 => {
            let tile_pos = TilePosition::new(column, rank);

            masks[tile_pos.bit_offset() as usize] = Pawn::generate_movement_mask(tile_pos, crate::player::Player::Black);
        });
    });

    masks
}
