use super::bitboard::Bitboard;

pub static mut ROOK_MASKS: Vec<Bitboard> = Vec::new();

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