use std::f32::consts::E;

use crate::{piece::Piece, player::Player};

use super::{bitboard::Bitboard, board::Board, move_mask::ROOK_MASKS};

pub fn get_collision_mask(board: Board, column: u32, rank: u32) -> Bitboard {
    let square_cont = board.get_piece(column, rank);

    if square_cont.is_none() {
        return Bitboard(0);
    }

    let (player, piece) = square_cont.unwrap();

    match piece {
        Piece::Pawn => return get_pawn_collision(player, column, rank),
        Piece::Rook => {
            return get_rook_collision(
                board,
                player,
                Bitboard::coordinates_to_bit_offset(column, rank),
            )
        }
        Piece::Bishop => return get_bishop_collision(player, column, rank),
        Piece::Knight => return get_knight_collision(player, column, rank),
        Piece::Queen => return get_queen_collision(player, column, rank),
        Piece::King => return get_king_collision(player, column, rank),
    }
}

/// Uses two masks to cut out movement after collision
pub fn get_rook_collision(board: Board, player: Player, offset: u32) -> Bitboard {
    unsafe {
        let mut valid_moves: u64 = ROOK_MASKS[offset as usize].value();
        let mut collision_mask: u64 =
            ((board.white_pieces | board.black_pieces) & valid_moves).into();

        if collision_mask != 0 {
            let rank_mask: u64 = 0xFE;

            let n_collision = get_column_mask(offset, 8 - offset / 8) & collision_mask;
            if n_collision != 0 {
                println!("Going to north checker");
                let n_offset = n_collision.trailing_zeros();
                if board
                    .get_player_bitboard(player.opposite())
                    .check_bit(n_offset)
                {
                    valid_moves &= !get_column_mask(n_offset + 8, 7 - (n_offset / 8));
                } else {
                    valid_moves &= !get_column_mask(n_offset, 8 - (n_offset / 8));
                }
                collision_mask &= !n_collision;
            }

            let w_collision = (rank_mask << offset) & collision_mask;
            if w_collision != 0 {
                let w_offset = w_collision.trailing_zeros();
                if board
                    .get_player_bitboard(player.opposite())
                    .check_bit(w_offset)
                {
                    valid_moves &= !(get_rank_mask(w_offset + 1, 7 - w_offset % 8));
                } else {
                    valid_moves &= !(get_rank_mask(w_offset, 8 - w_offset % 8));
                }
                collision_mask &= !w_collision;
            }

            let s_collision = get_column_mask(offset % 8, offset / 8) & collision_mask;
            if s_collision != 0 {
                println!("Going to south checker");
                let s_offset = 63 - s_collision.leading_zeros();
                if board
                    .get_player_bitboard(player.opposite())
                    .check_bit(s_offset)
                {
                    valid_moves &= !(get_column_mask(s_offset % 8, s_offset / 8));
                } else {
                    valid_moves &= !(get_column_mask(s_offset % 8, s_offset / 8 + 1));
                }
                collision_mask &= !s_collision;
            }

            if collision_mask != 0 {
                let e_offset = 63 - collision_mask.leading_zeros();
                println!("{}", collision_mask.leading_zeros());
                if board
                    .get_player_bitboard(player.opposite())
                    .check_bit(e_offset)
                {
                    valid_moves &= !(get_rank_mask(e_offset - e_offset % 8, e_offset % 8));
                } else {
                    valid_moves &= !(get_rank_mask(e_offset - e_offset % 8, e_offset % 8 + 1));
                }
            }
        }
        Bitboard(valid_moves)
    }
}

pub fn get_rank_mask(offset: u32, length: u32) -> u64 {
    println!("{}", offset);
    let (column, rank) = Bitboard::bit_offset_to_coordinates(offset);

    let mask_length = if length == 0 {
        return 0u64;
    } else {
        (1u64 << length) - 1
    };

    let rank_mask = mask_length << column;
    println!("{} {}", column, rank);
    rank_mask << (rank * 8)
}

pub fn get_column_mask(offset: u32, length: u32) -> u64 {
    let mut column_mask = 0u64;
    println!("{}", length);

    for i in 0..length {
        column_mask |= 1u64 << (offset + i * 8);
    }
    println!("Returning {}", column_mask);

    column_mask
}

pub fn get_bishop_collision(player: Player, column: u32, rank: u32) -> Bitboard {
    let bishop_mask = 0;
    Bitboard(bishop_mask)
}

pub fn get_knight_collision(player: Player, column: u32, rank: u32) -> Bitboard {
    let knight_mask = 0;
    Bitboard(knight_mask)
}

pub fn get_queen_collision(player: Player, column: u32, rank: u32) -> Bitboard {
    let queen_mask = 0;
    Bitboard(queen_mask)
}

pub fn get_king_collision(player: Player, column: u32, rank: u32) -> Bitboard {
    let king_mask = 0;
    Bitboard(king_mask)
}

pub fn get_pawn_collision(player: Player, column: u32, rank: u32) -> Bitboard {
    let pawn_mask = 0;
    Bitboard(pawn_mask)
}
