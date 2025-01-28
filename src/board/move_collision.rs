use crate::{piece::Piece, player::Player};

use super::{bitboard::Bitboard, board::Board, move_mask::{KNIGHT_MASKS, ROOK_MASKS}, tile_position::TilePosition};

pub fn get_collision_mask(board: Board, tile_pos: TilePosition) -> Bitboard {
    let square_cont = board.get_piece(tile_pos);

    if square_cont.is_none() {
        return Bitboard(0);
    }

    let piece = square_cont.unwrap();

    match piece.piece() {
        Piece::Pawn => return get_pawn_collision(piece.player(), tile_pos.column(), tile_pos.rank()),
        Piece::Rook => {
            return get_rook_collision(
                board,
                piece.player(),
                tile_pos.bit_offset(),
            )
        }
        Piece::Bishop => return get_bishop_collision(piece.player(), tile_pos.column(), tile_pos.rank()),
        Piece::Knight => return get_knight_collision(board, piece.player(), tile_pos.bit_offset()),
        Piece::Queen => return get_queen_collision(piece.player(), tile_pos.column(), tile_pos.rank()),
        Piece::King => return get_king_collision(piece.player(), tile_pos.column(), tile_pos.rank()),
    }
}

/// Uses two masks to cut out movement after collision
pub fn get_rook_collision(board: Board, player: Player, offset: u32) -> Bitboard {
    let mut valid_moves: u64 = ROOK_MASKS[offset as usize].value();

    let mut collision_mask: u64 =
        ((board.white_pieces | board.black_pieces) & valid_moves).into();

    if collision_mask == 0 {
        return Bitboard(valid_moves);
    };

    let rank_mask: u64 = 0xFE;

    let n_collision = get_column_mask(offset, 8 - offset / 8) & collision_mask;

    if n_collision != 0 {
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
        if board
            .get_player_bitboard(player.opposite())
            .check_bit(e_offset)
        {
            valid_moves &= !(get_rank_mask(e_offset - e_offset % 8, e_offset % 8));
        } else {
            valid_moves &= !(get_rank_mask(e_offset - e_offset % 8, e_offset % 8 + 1));
        }
    }

    Bitboard(valid_moves)
}

pub fn get_rank_mask(offset: u32, length: u32) -> u64 {
    let tile_pos = TilePosition::from_bit_offset(offset);

    let mask_length = if length == 0 {
        return 0u64;
    } else {
        (1u64 << length) - 1
    };

    let rank_mask = mask_length << tile_pos.column();
    rank_mask << (tile_pos.rank() * 8)
}

pub fn get_column_mask(offset: u32, length: u32) -> u64 {
    let mut column_mask = 0u64;

    for i in 0..length {
        column_mask |= 1u64 << (offset + i * 8);
    }

    column_mask
}

pub fn get_bishop_collision(player: Player, column: u32, rank: u32) -> Bitboard {
    let bishop_mask = 0;
    Bitboard(bishop_mask)
}

pub fn get_knight_collision(board: Board, player: Player, offset: u32) -> Bitboard {
    let knight_moves: u64 = KNIGHT_MASKS[offset as usize].value();
    let colliding_pieces = match player {
        Player::White => board.white_pieces.value(),
        Player::Black => board.black_pieces.value(),
    };
    Bitboard(knight_moves & !colliding_pieces)
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
