use std::cmp::min;

use crate::{piece::PieceType, pieces::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, rook::Rook}, player::Player};

use super::{
    bitboard::Bitboard,
    board::Board,
    tile_position::TilePosition,
};

pub fn get_collision_mask(board: Board, tile_pos: TilePosition) -> Bitboard {
    let square_cont = board.get_piece(tile_pos);

    if square_cont.is_none() {
        return Bitboard(0);
    }

    let piece = square_cont.unwrap();

    match piece.piece() {
        PieceType::Pawn => return get_pawn_collision(board, piece.player(), tile_pos),
        PieceType::Rook => return get_rook_collision(board, piece.player(), tile_pos.bit_offset()),
        PieceType::Bishop => return get_bishop_collision(board, piece.player(), tile_pos),
        PieceType::Knight => return get_knight_collision(board, piece.player(), tile_pos),
        PieceType::Queen => {
            return get_queen_collision(board, piece.player(), tile_pos)
        }
        PieceType::King => return get_king_collision(board, piece.player(), tile_pos),
    }
}

/// Uses two masks to cut out movement after collision
pub fn get_rook_collision(board: Board, player: Player, offset: u32) -> Bitboard {
    let mut valid_moves: u64 = Rook::MOVEMENT_MASKS[offset as usize].value();

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

pub fn get_bishop_collision(board: Board, player: Player, tile_pos: TilePosition) -> Bitboard {
    let mut valid_moves: u64 = Bishop::MOVEMENT_MASKS[tile_pos.bit_offset() as usize].value();
    let column = tile_pos.column();
    let rank = tile_pos.rank();
    let offset = tile_pos.bit_offset();

    let mut collision_mask: u64 = ((board.white_pieces | board.black_pieces) & valid_moves).value();

    if collision_mask == 0 {
        return Bitboard(valid_moves);
    }

    let ne_collision: u64 = get_cut_mask_asc(offset, min(8 - column, 8 - rank)) & collision_mask;

    if ne_collision != 0 {
        let ne_offset = TilePosition::from_bit_offset(ne_collision.trailing_zeros());
        if board
            .get_player_bitboard(player.opposite())
            .check_bit(ne_offset.bit_offset())
        {
            valid_moves &=
                !(get_cut_mask_asc(ne_offset.bit_offset(), min(7 - column, 7 - rank)) << 9);
        } else {
            valid_moves &= !get_cut_mask_asc(ne_offset.bit_offset(), min(8 - column, 8 - rank));
        }
        collision_mask &= !ne_collision;
    }

    let nw_collision: u64 = get_cut_mask_des(offset, min(column + 1, 8 - rank)) & collision_mask;
    if nw_collision != 0 {
        let nw_offset = TilePosition::from_bit_offset(nw_collision.trailing_zeros());
        if board
            .get_player_bitboard(player.opposite())
            .check_bit(nw_offset.bit_offset())
        {
            let distance = min(nw_offset.column(), 8 - nw_offset.rank());
            valid_moves &= !(get_cut_mask_des(nw_offset.bit_offset(), distance) << 7);
        } else {
            let distance = min(nw_offset.column(), 8 - nw_offset.rank());
            valid_moves &= !get_cut_mask_des(nw_offset.bit_offset(), distance + 1);
        }
        collision_mask &= !nw_collision;
    }

    let sw_collision = (Bitboard::get_diagonal_mask_asc(column, rank).value()) & collision_mask;
    if sw_collision != 0 {
        let sw_offset = TilePosition::from_bit_offset(63 - sw_collision.leading_zeros());
        if board
            .get_player_bitboard(player.opposite())
            .check_bit(sw_offset.bit_offset())
        {
            let distance = min(sw_offset.column(), sw_offset.rank());
            valid_moves &= !get_cut_mask_asc(sw_offset.bit_offset() - distance * 9, distance);
        } else {
            let distance = min(sw_offset.column(), sw_offset.rank());
            valid_moves &= !get_cut_mask_asc(sw_offset.bit_offset() - distance * 9, distance + 1);
        }
        collision_mask &= !sw_collision;
    }

    if collision_mask != 0 {
        let se_offset = TilePosition::from_bit_offset(63 - collision_mask.leading_zeros());
        if board
            .get_player_bitboard(player.opposite())
            .check_bit(se_offset.bit_offset())
        {
            let distance = min(8 - se_offset.column(), se_offset.rank());
            let attack_offset;
            if se_offset.bit_offset() > 7 {
                attack_offset = se_offset.bit_offset() - 7;
            } else {
                attack_offset = se_offset.bit_offset()
            };
            for i in 0..distance {
                valid_moves &= !(1 << attack_offset - 7 * i);
            }
        } else {
            let distance = min(8 - se_offset.column(), se_offset.rank() + 1);
            for i in 0..distance {
                valid_moves &= !(1 << se_offset.bit_offset() - 7 * i);
            }
        }
    }

    Bitboard(valid_moves)
}

pub fn get_cut_mask_asc(offset: u32, length: u32) -> u64 {
    let mut asc_mask = 0u64;
    for i in 0..length {
        asc_mask |= 1u64 << (i * 9);
    }
    asc_mask << offset
}

pub fn get_cut_mask_des(offset: u32, length: u32) -> u64 {
    let mut des_mask = 0u64;
    for i in 0..length {
        des_mask |= 128u64 << (i * 7)
    }

    des_mask >>= 7;
    des_mask << offset
}

pub fn get_knight_collision(board: Board, player: Player, tile_pos: TilePosition) -> Bitboard {
    Bitboard(
        Knight::MOVEMENT_MASKS[tile_pos.bit_offset() as usize].value()
            & !board.get_player_bitboard(player).value(),
    )
}

pub fn get_queen_collision(board: Board, player: Player, tile_pos: TilePosition) -> Bitboard {
    get_rook_collision(board.clone(), player, tile_pos.bit_offset()) | get_bishop_collision(board.clone(), player, tile_pos)
}

pub fn get_king_collision(board: Board, player: Player, tile_pos: TilePosition) -> Bitboard {
    Bitboard(
        King::MOVEMENT_MASKS[tile_pos.bit_offset() as usize].value()
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
                Pawn::WHITE_MOVEMENT_MASKS[tile_pos.bit_offset() as usize].value() & !collision_mask
            }
        }
        Player::Black => {
            if (1 << tile_pos.bit_offset() - 8) & collision_mask != 0 {
                0
            } else {
                Pawn::BLACK_MOVEMENT_MASKS[tile_pos.bit_offset() as usize].value() & !collision_mask
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
