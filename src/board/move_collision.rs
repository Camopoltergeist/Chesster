use crate::{
    piece::PieceType,
    pieces::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook},
    player::Player,
};

use const_for::const_for;

use super::{bitboard::Bitboard, board::Board, tile_position::TilePosition};

pub fn get_collision_mask(board: Board, tile_pos: TilePosition) -> Bitboard {
    let square_cont = board.get_piece(tile_pos);

    if square_cont.is_none() {
        return Bitboard(0);
    }

    let piece = square_cont.unwrap();

    match piece.piece() {
        PieceType::Pawn => return Pawn::generate_collision_mask(&board, piece.player(), tile_pos),
        PieceType::Rook => return Rook::generate_collision_mask(&board, piece.player(), tile_pos),
        PieceType::Bishop => {
            return Bishop::generate_collision_mask(&board, piece.player(), tile_pos)
        }
        PieceType::Knight => {
            return Knight::generate_collision_mask(&board, piece.player(), tile_pos)
        }
        PieceType::Queen => {
            return Queen::generate_collision_mask(&board, piece.player(), tile_pos)
        }
        PieceType::King => return King::generate_collision_mask(&board, piece.player(), tile_pos),
    }
}

pub fn n_collision_cut_mask(board: &Board, n_collision: Bitboard, player: Player) -> Bitboard {
    let first_collision = TilePosition::from_bit_offset(n_collision.value().trailing_zeros());

    if board
        .get_player_bitboard(player.opposite())
        .check_bit(first_collision.bit_offset())
    {
        if first_collision.rank() == 7 {
            Bitboard(0)
        } else {
            Bitboard::generate_column_mask(first_collision.column())
                << (first_collision.rank() as u64 + 1) * 8
        }
    } else {
        Bitboard::generate_column_mask(first_collision.column())
            << first_collision.rank() as u64 * 8
    }
}

pub fn e_collision_cut_mask(board: &Board, e_collision: Bitboard, player: Player) -> Bitboard {
    let first_collision = TilePosition::from_bit_offset(e_collision.value().trailing_zeros());

    if board
        .get_player_bitboard(player.opposite())
        .check_bit(first_collision.bit_offset())
    {
        if first_collision.column() == 7 {
            Bitboard(0)
        } else {
            Bitboard::generate_horizontal_line(7 - first_collision.column())
                << first_collision.bit_offset() as u64 + 1
        }
    } else {
        Bitboard::generate_horizontal_line(8 - first_collision.column())
            << first_collision.bit_offset() as u64
    }
}

pub fn s_collision_cut_mask(board: &Board, s_collision: Bitboard, player: Player) -> Bitboard {
    let first_collision = TilePosition::from_bit_offset(63 - s_collision.value().leading_zeros());

    if board
        .get_player_bitboard(player.opposite())
        .check_bit(first_collision.bit_offset())
    {
        if first_collision.rank() == 0 {
            Bitboard(0)
        } else {
            Bitboard::generate_column_mask(first_collision.column())
                >> (8 - first_collision.rank() as u64) * 8
        }
    } else {
        Bitboard::generate_column_mask(first_collision.column())
            >> (7 - first_collision.rank() as u64) * 8
    }
}

pub fn w_collision_cut_mask(board: &Board, w_collision: Bitboard, player: Player) -> Bitboard {
    let first_collision = TilePosition::from_bit_offset(63 - w_collision.value().leading_zeros());

    if board
        .get_player_bitboard(player.opposite())
        .check_bit(first_collision.bit_offset())
    {
        Bitboard::generate_horizontal_line(first_collision.column())
            << first_collision.rank() as u64 * 8
    } else {
        Bitboard::generate_horizontal_line(first_collision.column() + 1)
            << first_collision.rank() as u64 * 8
    }
}

pub fn ne_collision_cut_mask(board: &Board, ne_collision: Bitboard, player: Player) -> Bitboard {
    let first_collision = TilePosition::from_bit_offset(ne_collision.value().trailing_zeros());

    if board
        .get_player_bitboard(player.opposite())
        .check_bit(first_collision.bit_offset())
    {
        if first_collision.bit_offset() > 54 {
            Bitboard(0)
        } else {
            Bitboard::get_diagonal_mask_asc(first_collision.column(), first_collision.rank()) & !Bitboard::generate_horizontal_line(first_collision.bit_offset() + 9)
        }
    } else {
        Bitboard::get_diagonal_mask_asc(first_collision.column(), first_collision.rank()) & !Bitboard::generate_horizontal_line(first_collision.bit_offset())
    }
}

pub fn nw_collision_cut_mask(board: &Board, nw_collision: Bitboard, player: Player) -> Bitboard {
    let first_collision = TilePosition::from_bit_offset(nw_collision.value().trailing_zeros());

    if board
        .get_player_bitboard(player.opposite())
        .check_bit(first_collision.bit_offset())
    {
        if first_collision.rank() == 7 {
            Bitboard(0)
        } else {
            Bitboard::get_diagonal_mask_des(first_collision.column(), first_collision.rank()) & !Bitboard::generate_horizontal_line(first_collision.bit_offset() + 7)
        }
    } else {
        Bitboard::get_diagonal_mask_des(first_collision.column(), first_collision.rank()) & !Bitboard::generate_horizontal_line(first_collision.bit_offset())
    }
}

pub fn sw_collision_cut_mask(board: &Board, sw_collision: Bitboard, player: Player) -> Bitboard {
    let first_collision = TilePosition::from_bit_offset(63 - sw_collision.value().leading_zeros());

    if board
        .get_player_bitboard(player.opposite())
        .check_bit(first_collision.bit_offset())
    {
        if first_collision.bit_offset() < 9 {
            Bitboard(0)
        } else {
            Bitboard::get_diagonal_mask_asc(first_collision.column(), first_collision.rank()) & Bitboard::generate_horizontal_line(first_collision.bit_offset())
        }
    } else {
        Bitboard::get_diagonal_mask_asc(first_collision.column(), first_collision.rank()) & Bitboard::generate_horizontal_line(first_collision.bit_offset() + 1)
    }
}

pub fn se_collision_cut_mask(board: &Board, se_collision: Bitboard, player: Player) -> Bitboard {
    let first_collision = TilePosition::from_bit_offset(63 - se_collision.value().leading_zeros());

    if board
        .get_player_bitboard(player.opposite())
        .check_bit(first_collision.bit_offset())
    {
        if first_collision.bit_offset() < 7 {
            Bitboard(0)
        } else {
            Bitboard::get_diagonal_mask_des(first_collision.column(), first_collision.rank()) & Bitboard::generate_horizontal_line(first_collision.bit_offset())

        }
    } else {
        Bitboard::get_diagonal_mask_des(first_collision.column(), first_collision.rank()) & Bitboard::generate_horizontal_line(first_collision.bit_offset() + 1)
    }
}

pub const fn get_pawn_capture(player: Player, tile_pos: TilePosition) -> u64 {
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
