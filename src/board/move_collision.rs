use crate::{piece::Piece, player::Player};

use super::{
    bitboard::Bitboard,
    board::Board,
    move_mask::{BLACK_PAWN_MASKS, KING_MASKS, KNIGHT_MASKS, ROOK_MASKS, WHITE_PAWN_MASKS},
    tile_position::TilePosition,
};

pub fn get_collision_mask(board: Board, tile_pos: TilePosition) -> Bitboard {
    let square_cont = board.get_piece(tile_pos);

    if square_cont.is_none() {
        return Bitboard(0);
    }

    let piece = square_cont.unwrap();

    match piece.piece() {
        Piece::Pawn => return get_pawn_collision(board, piece.player(), tile_pos),
        Piece::Rook => return get_rook_collision(board, piece.player(), tile_pos.bit_offset()),
        Piece::Bishop => {
            return get_bishop_collision(piece.player(), tile_pos.column(), tile_pos.rank())
        }
        Piece::Knight => return get_knight_collision(board, piece.player(), tile_pos),
        Piece::Queen => {
            return get_queen_collision(piece.player(), tile_pos.column(), tile_pos.rank())
        }
        Piece::King => return get_king_collision(board, piece.player(), tile_pos),
    }
}

/// Uses two masks to cut out movement after collision
pub fn get_rook_collision(board: Board, player: Player, offset: u32) -> Bitboard {
    let mut valid_moves: u64 = ROOK_MASKS[offset as usize].value();

    let mut collision_mask: u64 = ((board.white_pieces | board.black_pieces) & valid_moves).into();

    if collision_mask == 0 {
        return Bitboard(valid_moves);
    };

    let rank_mask: u64 = 0xFE;

    let n_collision = get_cut_mask_vertical(offset, 8 - offset / 8) & collision_mask;

    if n_collision != 0 {
        let n_offset = n_collision.trailing_zeros();
        if board
            .get_player_bitboard(player.opposite())
            .check_bit(n_offset)
        {
            valid_moves &= !get_cut_mask_vertical(n_offset + 8, 7 - (n_offset / 8));
        } else {
            valid_moves &= !get_cut_mask_vertical(n_offset, 8 - (n_offset / 8));
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
            valid_moves &= !(get_cut_mask_horizontal(w_offset + 1, 7 - w_offset % 8));
        } else {
            valid_moves &= !(get_cut_mask_horizontal(w_offset, 8 - w_offset % 8));
        }
        collision_mask &= !w_collision;
    }

    let s_collision = get_cut_mask_vertical(offset % 8, offset / 8) & collision_mask;

    if s_collision != 0 {
        let s_offset = 63 - s_collision.leading_zeros();
        if board
            .get_player_bitboard(player.opposite())
            .check_bit(s_offset)
        {
            valid_moves &= !(get_cut_mask_vertical(s_offset % 8, s_offset / 8));
        } else {
            valid_moves &= !(get_cut_mask_vertical(s_offset % 8, s_offset / 8 + 1));
        }
        collision_mask &= !s_collision;
    }

    if collision_mask != 0 {
        let e_offset = 63 - collision_mask.leading_zeros();
        if board
            .get_player_bitboard(player.opposite())
            .check_bit(e_offset)
        {
            valid_moves &= !(get_cut_mask_horizontal(e_offset - e_offset % 8, e_offset % 8));
        } else {
            valid_moves &= !(get_cut_mask_horizontal(e_offset - e_offset % 8, e_offset % 8 + 1));
        }
    }

    Bitboard(valid_moves)
}

pub fn get_cut_mask_horizontal(offset: u32, length: u32) -> u64 {
    let tile_pos = TilePosition::from_bit_offset(offset);

    let mask_length = if length == 0 {
        return 0u64;
    } else {
        (1u64 << length) - 1
    };

    let rank_mask = mask_length << tile_pos.column();
    rank_mask << (tile_pos.rank() * 8)
}

pub fn get_cut_mask_vertical(offset: u32, length: u32) -> u64 {
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

pub fn get_knight_collision(board: Board, player: Player, tile_pos: TilePosition) -> Bitboard {
    Bitboard(
        KNIGHT_MASKS[tile_pos.bit_offset() as usize].value()
            & !board.get_player_bitboard(player).value(),
    )
}

pub fn get_queen_collision(player: Player, column: u32, rank: u32) -> Bitboard {
    let queen_mask = 0;
    Bitboard(queen_mask)
}

pub fn get_king_collision(board: Board, player: Player, tile_pos: TilePosition) -> Bitboard {
    Bitboard(
        KING_MASKS[tile_pos.bit_offset() as usize].value()
            & !board.get_player_bitboard(player).value(),
    )
}

pub fn get_pawn_collision(board: Board, player: Player, tile_pos: TilePosition) -> Bitboard {
    let collision_mask = (board.white_pieces | board.black_pieces).value();

    let valid_moves = match player {
        Player::White => {
            if (1 << tile_pos.bit_offset() + 8) & collision_mask != 0 {
                0
            } else {
                WHITE_PAWN_MASKS[tile_pos.bit_offset() as usize].value() & !collision_mask
            }
        }
        Player::Black => {
            if (1 << tile_pos.bit_offset() - 8) & collision_mask != 0 {
                0
            } else {
                BLACK_PAWN_MASKS[tile_pos.bit_offset() as usize].value() & !collision_mask
            }
        }
    };

    let capture_moves =
        get_pawn_capture(player, tile_pos) & board.get_player_bitboard(player.opposite()).value();
    Bitboard(valid_moves | capture_moves)
}

pub fn get_pawn_capture(player: Player, tile_pos: TilePosition) -> u64 {
    let mut attack_tiles = 0;
    let attack_mask = 1u64 << tile_pos.bit_offset();

    match player {
        Player::White => {
            if tile_pos.column() != 0 {
                attack_tiles |= attack_mask << 7;
            }
            if tile_pos.column() != 7 {
                attack_tiles |= attack_mask << 9;
            }
        }
        Player::Black => {
            if tile_pos.column() != 0 {
                attack_tiles |= attack_mask >> 9;
            }
            if tile_pos.column() != 7 {
                attack_tiles |= attack_mask >> 7;
            }
        }
    }

    attack_tiles
}
