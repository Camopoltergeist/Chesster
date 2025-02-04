
use crate::{piece::PieceType, pieces::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook}, player::Player};

use const_for::const_for;

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
        PieceType::Pawn => return Pawn::generate_collision_mask(&board, piece.player(), tile_pos),
        PieceType::Rook => return Rook::generate_collision_mask(&board, piece.player(), tile_pos),
        PieceType::Bishop => return Bishop::generate_collision_mask(&board, piece.player(), tile_pos),
        PieceType::Knight => return Knight::generate_collision_mask(&board, piece.player(), tile_pos),
        PieceType::Queen => {
            return Queen::generate_collision_mask(&board, piece.player(), tile_pos)
        }
        PieceType::King => return King::generate_collision_mask(&board, piece.player(), tile_pos),
    }
}

pub const fn get_cut_mask_horizontal(offset: u32, length: u32) -> u64 {
    let tile_pos = TilePosition::from_bit_offset(offset);

    let mask_length = if length == 0 {
        return 0u64;
    } else {
        (1u64 << length) - 1
    };

    let rank_mask = mask_length << tile_pos.column();
    rank_mask << (tile_pos.rank() * 8)
}

pub const fn get_cut_mask_vertical(offset: u32, length: u32) -> u64 {
    let mut column_mask = 0u64;

    const_for!(i in 0..length => {
        column_mask |= 1u64 << (offset + i * 8);
    });

    column_mask
}

pub const fn get_cut_mask_asc(offset: u32, length: u32) -> u64 {
    let mut asc_mask = 0u64;
    
    const_for!(i in 0..length => {
        asc_mask |= 1u64 << (i * 9);
    });

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
