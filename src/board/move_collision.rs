use crate::piece::Piece;

use super::{bitboard::Bitboard, board::Board};

pub fn get_collision_mask(board: Board, column: u32, rank: u32) -> Bitboard {
    let square_cont = board.get_piece(column, rank);

    if square_cont.is_none() {
        return Bitboard(0);
    }

    let (player, piece) = square_cont.unwrap();

    match piece {
        Piece::Pawn => get_pawn_collision(),
        Piece::Rook => get_rook_collision(),
        Piece::Bishop => get_bishop_collision(),
        Piece::Knight => get_knight_collision(),
        Piece::Queen => get_queen_collision(),
        Piece::King => get_king_collision(),
    }

    let collision_mask = 0;
    Bitboard(collision_mask)
}

pub fn get_rook_collision() {

}

pub fn get_bishop_collision() {

}

pub fn get_knight_collision() {

}

pub fn get_queen_collision() {

}

pub fn get_king_collision() {

}

pub fn get_pawn_collision() {

}