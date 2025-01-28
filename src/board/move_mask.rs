use crate::{board::tile_position::TilePosition, piece::Piece, player::Player, player_piece::PlayerPiece};

use super::bitboard::Bitboard;

use const_for::const_for;

pub const ROOK_MASKS: [Bitboard; 64] = generate_rook_masks();
pub const BISHOP_MASKS: [Bitboard; 64] = generate_bishop_masks();
pub const KNIGHT_MASKS: [Bitboard; 64] = generate_knight_masks();
pub const KING_MASKS: [Bitboard; 64] = generate_king_masks();
pub const QUEEN_MASKS: [Bitboard; 64] = generate_queen_masks();
pub const WHITE_PAWN_MASKS: [Bitboard; 64] = generate_white_pawn_masks();
pub const BLACK_PAWN_MASKS: [Bitboard; 64] = generate_black_pawn_masks();

pub fn get_move_mask(piece: PlayerPiece) -> &'static [Bitboard] {
    match piece.piece() {
        Piece::Rook => &ROOK_MASKS,
        Piece::Bishop => &BISHOP_MASKS,
        Piece::Knight => &KNIGHT_MASKS,
        Piece::King => &KING_MASKS,
        Piece::Queen => &QUEEN_MASKS,
        Piece::Pawn => {
            match piece.player() {
                Player::White => &WHITE_PAWN_MASKS,
                Player::Black => &BLACK_PAWN_MASKS
            }
        }
    }
}

pub const fn generate_rook_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];

    const_for!(rank in 0..8 => {
        let rank_mask = Bitboard::get_rank_mask(rank);

        const_for!(column in 0..8 => {
            let column_mask = Bitboard::get_column_mask(column);

            let combined = column_mask.0 ^ rank_mask.0;

            let index = TilePosition::new(column, rank).bit_offset();

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

            let index = TilePosition::new(column, rank).bit_offset();

            masks[index as usize] = Bitboard(combined);
        })
    });

    masks
}

pub const fn generate_knight_masks() -> [Bitboard; 64] {
    let mut masks = [Bitboard(0); 64];

    const_for!(rank in 0..8 => {
        const_for!(column in 0..8 => {
            let index = TilePosition::new(column, rank).bit_offset();

            masks[index as usize] = Bitboard::get_knight_mask(column, rank);
        });
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

    const_for!(i in 0..ROOK_MASKS.len() => {
        let rook_mask = ROOK_MASKS[i];
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
