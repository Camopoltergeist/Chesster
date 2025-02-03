use crate::board::tile_position::TilePosition;

use super::bitboard::Bitboard;

use const_for::const_for;

pub const WHITE_PAWN_MASKS: [Bitboard; 64] = generate_white_pawn_masks();
pub const BLACK_PAWN_MASKS: [Bitboard; 64] = generate_black_pawn_masks();

pub const fn generate_white_pawn_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];

    const_for!(rank in 0..8 => {
        const_for!(column in 0..8 => {
            let index = TilePosition::new(column, rank).bit_offset();

            masks[index as usize] = Bitboard::get_white_pawn_mask(column, rank);
        });
    });

    masks
}

pub const fn generate_black_pawn_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];

    const_for!(rank in 0..8 => {
        const_for!(column in 0..8 => {
            let index = TilePosition::new(column, rank).bit_offset();

            masks[index as usize] = Bitboard::get_black_pawn_mask(column, rank);
        });
    });

    masks
}
