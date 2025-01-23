use raylib::ffi::PI;

use super::{
    bitboard::Bitboard,
    piece::{self, Piece},
};
use crate::player::{self, Player};

pub struct Board {
    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,
    pub pawns: Bitboard,
    pub rooks: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub queens: Bitboard,
    pub kings: Bitboard,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            white_pieces: Bitboard(
                0b_0000000_0000000_0000000_0000000_0000000_00000000_11111111_11111111,
            ),
            black_pieces: Bitboard(
                0b_11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
            ),
            pawns: Bitboard(
                0b_00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000,
            ),
            rooks: Bitboard(
                0b_10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            ),
            knights: Bitboard(
                0b_01000010_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            ),
            bishops: Bitboard(
                0b_00100100_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            ),
            queens: Bitboard(
                0b_00001000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            ),
            kings: Bitboard(
                0b_00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            ),
        }
    }
}

struct GetPieceResult(Player, Piece);

impl Board {
    pub fn check_overlaps(a: Bitboard, b: Bitboard) -> bool {
        a.0 & b.0 != 0
    }

    pub fn validate(&self) -> bool {
        !(Board::check_overlaps(self.pawns, self.rooks)
            || Board::check_overlaps(self.pawns, self.knights)
            || Board::check_overlaps(self.pawns, self.bishops)
            || Board::check_overlaps(self.pawns, self.queens)
            || Board::check_overlaps(self.pawns, self.kings)
            || Board::check_overlaps(self.rooks, self.knights)
            || Board::check_overlaps(self.rooks, self.bishops)
            || Board::check_overlaps(self.rooks, self.queens)
            || Board::check_overlaps(self.rooks, self.kings)
            || Board::check_overlaps(self.knights, self.bishops)
            || Board::check_overlaps(self.knights, self.queens)
            || Board::check_overlaps(self.knights, self.kings)
            || Board::check_overlaps(self.bishops, self.queens)
            || Board::check_overlaps(self.bishops, self.kings)
            || Board::check_overlaps(self.queens, self.kings))
    }

    pub fn move_piece(&mut self, player: player::Player, piece: Piece, from: u32, to: u32) {
        //Checks the player's color and edits both that color's and a piece's bitboard
        //Every piece's possible move mask(s) probably need to also be implemented l8r
        match piece {
            Piece::Pawn => Bitboard::move_bit(&mut self.pawns, from, to),
            Piece::Rook => Bitboard::move_bit(&mut self.rooks, from, to),
            Piece::Knight => Bitboard::move_bit(&mut self.knights, from, to),
            Piece::Bishop => Bitboard::move_bit(&mut self.bishops, from, to),
            Piece::Queen => Bitboard::move_bit(&mut self.queens, from, to),
            Piece::King => Bitboard::move_bit(&mut self.kings, from, to),
        }

        match player {
            Player::White => Bitboard::move_bit(&mut self.white_pieces, from, to),
            Player::Black => Bitboard::move_bit(&mut self.black_pieces, from, to),
        }
    }

    pub fn get_piece(&self, index: u32) -> Option<(Player, Piece)> {
        //Not ready: how to implement empty spaces, should there be a None in enum? is matching the way to go?
        //Or Option<(Player, Piece)>?
        let player = if Bitboard::check_bit(&self.white_pieces, index) {
            {
                Player::White
            }
        } else if Bitboard::check_bit(&self.black_pieces, index) {
            {
                Player::Black
            }
        } else {
            return None;
        };

        let piece = match () {
            _ if Bitboard::check_bit(&self.pawns, index) => Piece::Pawn,
            _ if Bitboard::check_bit(&self.rooks, index) => Piece::Rook,
            _ if Bitboard::check_bit(&self.knights, index) => Piece::Knight,
            _ if Bitboard::check_bit(&self.bishops, index) => Piece::Bishop,
            _ if Bitboard::check_bit(&self.queens, index) => Piece::Queen,
            _ if Bitboard::check_bit(&self.kings, index) => Piece::King,
            _ => return None,
        };

        Some((player, piece))
    }
}
