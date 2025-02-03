use crate::{board::tile_position::TilePosition, pieces::{bishop::Bishop, rook::Rook}};

use super::bitboard::Bitboard;

use const_for::const_for;

pub const BISHOP_MASKS: [Bitboard; 64] = generate_bishop_masks();
pub const KING_MASKS: [Bitboard; 64] = generate_king_masks();
pub const QUEEN_MASKS: [Bitboard; 64] = generate_queen_masks();
pub const WHITE_PAWN_MASKS: [Bitboard; 64] = generate_white_pawn_masks();
pub const BLACK_PAWN_MASKS: [Bitboard; 64] = generate_black_pawn_masks();

pub const fn generate_bishop_masks() -> [Bitboard; 64] {
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

pub const fn generate_king_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];

    const_for!(rank in 0..8 => {
        const_for!(column in 0..8 => {
            let index = TilePosition::new(column, rank).bit_offset();

            masks[index as usize] = Bitboard::get_king_mask(column, rank);
        });
    });

    masks
}

pub const fn generate_queen_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];

    const_for!(i in 0..Rook::MOVEMENT_MASKS.len() => {
        let rook_mask = Rook::MOVEMENT_MASKS[i];
        let bishop_mask = BISHOP_MASKS[i];

        let combined = rook_mask.0 | bishop_mask.0;

        masks[i] = Bitboard(combined);
    });

    masks
}

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
