use super::bitboard::Bitboard;

pub static mut ROOK_MASKS: Vec<Bitboard> = Vec::new();
pub static mut BISHOP_MASKS: Vec<Bitboard> = Vec::new();
pub static mut KNIGHT_MASKS: Vec<Bitboard> = Vec::new();
pub static mut KING_MASKS: Vec<Bitboard> = Vec::new();
pub static mut QUEEN_MASKS: Vec<Bitboard> = Vec::new();

pub fn generate_rook_masks() {
    for i in 0..8 {
        let rank_mask = Bitboard::get_rank_mask(i);

        for j in 0..8 {
            let column_mask = Bitboard::get_column_mask(j);

            let combined = column_mask.0 ^ rank_mask.0;

            unsafe {
                ROOK_MASKS.push(Bitboard(combined));
            }
        }
    }
}

pub fn generate_bishop_masks() {
    for i in 0..8 {
        for j in 0..8 {
            let rank_mask = Bitboard::get_diagonal_mask_asc(j, i);
            let column_mask = Bitboard::get_diagonal_mask_des(j, i);

            let combined = rank_mask.0 ^ column_mask.0;

            unsafe {
                BISHOP_MASKS.push(Bitboard(combined));
            }
        }
    }
}

pub fn generate_knight_masks() {
    for i in 0..8 {
        for j in 0..8 {
            let knight_mask = Bitboard::get_knight_mask(j, i);

            unsafe {
                KNIGHT_MASKS.push(knight_mask);
            }
        }
    }
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

