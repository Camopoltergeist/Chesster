use super::bitboard::Bitboard;
use crate::{piece::Piece, player::Player};

#[derive(Clone)]
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

impl Board {
    pub fn check_overlaps(a: Bitboard, b: Bitboard) -> bool {
        a & b != 0
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

    pub fn move_piece(
        &mut self,
        player: Player,
        piece: Piece,
        from_offset: u32,
        to_offset: u32,
    ) {
        self.get_piece_bitboard_mut(piece).move_bit(from_offset, to_offset);
        self.get_player_bitboard_mut(player).move_bit(from_offset, to_offset);
    }

    pub fn get_piece_from_offset(&self, bit_offset: u32) -> Option<(Player, Piece)> {
        let player = if Bitboard::check_bit(&self.white_pieces, bit_offset) {
            Player::White
        } else if Bitboard::check_bit(&self.black_pieces, bit_offset) {
            Player::Black
        } else {
            return None;
        };

        let piece = match () {
            _ if Bitboard::check_bit(&self.pawns, bit_offset) => Piece::Pawn,
            _ if Bitboard::check_bit(&self.rooks, bit_offset) => Piece::Rook,
            _ if Bitboard::check_bit(&self.knights, bit_offset) => Piece::Knight,
            _ if Bitboard::check_bit(&self.bishops, bit_offset) => Piece::Bishop,
            _ if Bitboard::check_bit(&self.queens, bit_offset) => Piece::Queen,
            _ if Bitboard::check_bit(&self.kings, bit_offset) => Piece::King,
            _ => return None,
        };

        Some((player, piece))
    }

    pub fn get_player_bitboard(&self, player: Player) -> &Bitboard {
        match player {
            Player::White => &self.white_pieces,
            Player::Black => &self.black_pieces
        }
    }

    pub fn get_player_bitboard_mut(&mut self, player: Player) -> &mut Bitboard {
        match player {
            Player::White => &mut self.white_pieces,
            Player::Black => &mut self.black_pieces
        }
    }

    pub fn get_piece_bitboard(&self, piece: Piece) -> &Bitboard {
        match piece {
            Piece::Pawn => &self.pawns,
            Piece::Rook => &self.rooks,
            Piece::Knight => &self.knights,
            Piece::Bishop => &self.bishops,
            Piece::Queen => &self.kings,
            Piece::King => &self.kings
        }
    }

    pub fn get_piece_bitboard_mut(&mut self, piece: Piece) -> &mut Bitboard {
        match piece {
            Piece::Pawn => &mut self.pawns,
            Piece::Rook => &mut self.rooks,
            Piece::Knight => &mut self.knights,
            Piece::Bishop => &mut self.bishops,
            Piece::Queen => &mut self.queens,
            Piece::King => &mut self.kings
        }
    }

    pub fn remove_piece_from_offset(&mut self, bit_offset: u32) {
        self.white_pieces.unset_bit(bit_offset);
        self.black_pieces.unset_bit(bit_offset);

        self.pawns.unset_bit(bit_offset);
        self.rooks.unset_bit(bit_offset);
        self.knights.unset_bit(bit_offset);
        self.bishops.unset_bit(bit_offset);
        self.queens.unset_bit(bit_offset);
        self.kings.unset_bit(bit_offset);
    }

    pub fn set_piece_to_offset(&mut self, player: Player, piece: Piece, bit_offset: u32) {
        self.remove_piece_from_offset(bit_offset);

        self.get_player_bitboard_mut(player).set_bit(bit_offset);
        self.get_piece_bitboard_mut(piece).set_bit(bit_offset);
    }

    pub fn get_piece(&self, column: u32, rank: u32) -> Option<(Player, Piece)> {
        let bit_offset = Bitboard::coordinates_to_bit_offset(column, rank);
        return self.get_piece_from_offset(bit_offset);
    }

    pub fn remove_piece(&mut self, column: u32, rank: u32) {
        let bit_offset = Bitboard::coordinates_to_bit_offset(column, rank);
        self.remove_piece_from_offset(bit_offset);
    }

    pub fn set_piece(&mut self, player: Player, piece: Piece, column: u32, rank: u32) {
        let bit_offset = Bitboard::coordinates_to_bit_offset(column, rank);
        self.set_piece_to_offset(player, piece, bit_offset);
    }
}
