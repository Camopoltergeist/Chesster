use super::{bitboard::Bitboard, board::Board};

use const_for::const_for;

pub const ROOK_MASKS: [Bitboard; 64] = generate_rook_masks();
pub const BISHOP_MASKS: [Bitboard; 64] = generate_bishop_masks();
pub const KNIGHT_MASKS: [Bitboard; 64] = generate_knight_masks();
pub static mut KING_MASKS: Vec<Bitboard> = Vec::new();
pub static mut QUEEN_MASKS: Vec<Bitboard> = Vec::new();
pub static mut WHITE_PAWN_MASKS: Vec<Bitboard> = Vec::new();
pub static mut BLACK_PAWN_MASKS: Vec<Bitboard> = Vec::new();

pub fn generate_masks() {
    generate_pawn_masks();
    generate_rook_masks();
    generate_bishop_masks();
    generate_knight_masks();
    generate_king_masks();
    generate_queen_masks();
}

pub const fn generate_rook_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];

    const_for!(rank in 0..8 => {
        let rank_mask = Bitboard::get_rank_mask(rank);

        const_for!(column in 0..8 => {
            let column_mask = Bitboard::get_column_mask(column);

            let combined = column_mask.0 ^ rank_mask.0;

            let index = Bitboard::coordinates_to_bit_offset(column as u32, rank as u32);

            masks[index as usize] = Bitboard(combined);
        })
    });

    masks
}

pub const fn generate_bishop_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];

    const_for!(rank in 0..8 => {
        const_for!(column in 0..8 => {
            let rank_mask = Bitboard::get_diagonal_mask_asc(column, rank);
            let column_mask = Bitboard::get_diagonal_mask_des(column, rank);

            let combined = rank_mask.0 ^ column_mask.0;

            let index = Bitboard::coordinates_to_bit_offset(column as u32, rank as u32);

            masks[index as usize] = Bitboard(combined);
        })
    });

    masks
}

pub const fn generate_knight_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];

    const_for!(rank in 0..8 => {
        const_for!(column in 0..8 => {
            let index = Bitboard::coordinates_to_bit_offset(column, rank);

            masks[index as usize] = Bitboard::get_knight_mask(column, rank);
        });
    });

    masks
}

pub fn generate_king_masks() {
    for i in 0..8 {
        for j in 0..8 {
            let king_mask = Bitboard::get_king_mask(j, i);

            unsafe {
                KING_MASKS.push(king_mask);
            }
        }
    }
}

pub fn generate_queen_masks() {
    unsafe {
        for i in 0..ROOK_MASKS.len() {
            let rook_mask = ROOK_MASKS[i];
            let bishop_mask = BISHOP_MASKS[i];

            let combined = rook_mask | bishop_mask;
            QUEEN_MASKS.push(combined);
        }
    }
}

pub fn generate_pawn_masks() {
    for i in 0..8 {
        for j in 0..8 {
            let white_pawn = Bitboard::get_white_pawn_mask(j, i);
            let black_pawn = Bitboard::get_black_pawn_mask(j, i);

            unsafe {
                WHITE_PAWN_MASKS.push(white_pawn);
                BLACK_PAWN_MASKS.push(black_pawn);
            }

        }
    }
}
