use super::{bitboard::Bitboard, tile_position::TilePosition};
use crate::{piece::Piece, player::Player, player_piece::PlayerPiece};

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
    pub fn empty() -> Self {
        Self {
            white_pieces: Bitboard(0),
            black_pieces: Bitboard(0),
            pawns: Bitboard(0),
            rooks: Bitboard(0),
            kings: Bitboard(0),
            knights: Bitboard(0),
            bishops: Bitboard(0),
            queens: Bitboard(0)
        }
    }

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

    pub fn move_piece(&mut self, player: Player, piece: Piece, from_offset: u32, to_offset: u32) {
        self.get_piece_bitboard_mut(piece)
            .move_bit(from_offset, to_offset);
        self.get_player_bitboard_mut(player)
            .move_bit(from_offset, to_offset);
    }

    pub fn get_piece_from_offset(&self, bit_offset: u32) -> Option<PlayerPiece> {
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

        Some(PlayerPiece::new(player, piece))
    }

    pub fn get_player_bitboard(&self, player: Player) -> &Bitboard {
        match player {
            Player::White => &self.white_pieces,
            Player::Black => &self.black_pieces,
        }
    }

    pub fn get_player_bitboard_mut(&mut self, player: Player) -> &mut Bitboard {
        match player {
            Player::White => &mut self.white_pieces,
            Player::Black => &mut self.black_pieces,
        }
    }

    pub fn get_piece_bitboard(&self, piece: Piece) -> &Bitboard {
        match piece {
            Piece::Pawn => &self.pawns,
            Piece::Rook => &self.rooks,
            Piece::Knight => &self.knights,
            Piece::Bishop => &self.bishops,
            Piece::Queen => &self.kings,
            Piece::King => &self.kings,
        }
    }

    pub fn get_piece_bitboard_mut(&mut self, piece: Piece) -> &mut Bitboard {
        match piece {
            Piece::Pawn => &mut self.pawns,
            Piece::Rook => &mut self.rooks,
            Piece::Knight => &mut self.knights,
            Piece::Bishop => &mut self.bishops,
            Piece::Queen => &mut self.queens,
            Piece::King => &mut self.kings,
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

    pub fn set_piece_to_offset(&mut self, piece: PlayerPiece, bit_offset: u32) {
        self.remove_piece_from_offset(bit_offset);

        self.get_player_bitboard_mut(piece.player()).set_bit(bit_offset);
        self.get_piece_bitboard_mut(piece.piece()).set_bit(bit_offset);
    }

    pub fn get_piece(&self, tile_pos: TilePosition) -> Option<PlayerPiece> {
        return self.get_piece_from_offset(tile_pos.bit_offset());
    }

    pub fn remove_piece(&mut self, tile_pos: TilePosition) {
        self.remove_piece_from_offset(tile_pos.bit_offset());
    }

    pub fn set_piece(&mut self, piece: PlayerPiece, tile_pos: TilePosition) {
        self.set_piece_to_offset(piece, tile_pos.bit_offset());
    }

    pub fn get_piece_debug(&self, tile_str: &str) -> Option<PlayerPiece> {
        let tile_pos = TilePosition::from_tile_str(tile_str).expect("invalid tile str passed");

        self.get_piece(tile_pos)
    }
}

#[cfg(test)]
mod tests {
    use crate::{piece::Piece, player::Player, player_piece::PlayerPiece};

    use super::Board;

    #[test]
    fn starting_position_has_no_overlaps() {
        let board = Board::default();

        assert!(board.validate());
    }

    #[test]
    fn starting_position_is_correct() {
        let board = Board::default();

        // Rank 1
        assert_eq!(board.get_piece_debug("a1"), Some(PlayerPiece::new(Player::White, Piece::Rook)));
        assert_eq!(board.get_piece_debug("b1"), Some(PlayerPiece::new(Player::White, Piece::Knight)));
        assert_eq!(board.get_piece_debug("c1"), Some(PlayerPiece::new(Player::White, Piece::Bishop)));
        assert_eq!(board.get_piece_debug("d1"), Some(PlayerPiece::new(Player::White, Piece::Queen)));

        assert_eq!(board.get_piece_debug("e1"), Some(PlayerPiece::new(Player::White, Piece::King)));
        assert_eq!(board.get_piece_debug("f1"), Some(PlayerPiece::new(Player::White, Piece::Bishop)));
        assert_eq!(board.get_piece_debug("g1"), Some(PlayerPiece::new(Player::White, Piece::Knight)));
        assert_eq!(board.get_piece_debug("h1"), Some(PlayerPiece::new(Player::White, Piece::Rook)));

        // Rank 2
        assert_eq!(board.get_piece_debug("a2"), Some(PlayerPiece::new(Player::White, Piece::Pawn)));
        assert_eq!(board.get_piece_debug("b2"), Some(PlayerPiece::new(Player::White, Piece::Pawn)));
        assert_eq!(board.get_piece_debug("c2"), Some(PlayerPiece::new(Player::White, Piece::Pawn)));
        assert_eq!(board.get_piece_debug("d2"), Some(PlayerPiece::new(Player::White, Piece::Pawn)));

        assert_eq!(board.get_piece_debug("e2"), Some(PlayerPiece::new(Player::White, Piece::Pawn)));
        assert_eq!(board.get_piece_debug("f2"), Some(PlayerPiece::new(Player::White, Piece::Pawn)));
        assert_eq!(board.get_piece_debug("g2"), Some(PlayerPiece::new(Player::White, Piece::Pawn)));
        assert_eq!(board.get_piece_debug("h2"), Some(PlayerPiece::new(Player::White, Piece::Pawn)));

        // Rank 3
        assert_eq!(board.get_piece_debug("a3"), None);
        assert_eq!(board.get_piece_debug("b3"), None);
        assert_eq!(board.get_piece_debug("c3"), None);
        assert_eq!(board.get_piece_debug("d3"), None);

        assert_eq!(board.get_piece_debug("e3"), None);
        assert_eq!(board.get_piece_debug("f3"), None);
        assert_eq!(board.get_piece_debug("g3"), None);
        assert_eq!(board.get_piece_debug("h3"), None);

        // Rank 4
        assert_eq!(board.get_piece_debug("a4"), None);
        assert_eq!(board.get_piece_debug("b4"), None);
        assert_eq!(board.get_piece_debug("c4"), None);
        assert_eq!(board.get_piece_debug("d4"), None);

        assert_eq!(board.get_piece_debug("e4"), None);
        assert_eq!(board.get_piece_debug("f4"), None);
        assert_eq!(board.get_piece_debug("g4"), None);
        assert_eq!(board.get_piece_debug("h4"), None);

        // Rank 5
        assert_eq!(board.get_piece_debug("a5"), None);
        assert_eq!(board.get_piece_debug("b5"), None);
        assert_eq!(board.get_piece_debug("c5"), None);
        assert_eq!(board.get_piece_debug("d5"), None);

        assert_eq!(board.get_piece_debug("e5"), None);
        assert_eq!(board.get_piece_debug("f5"), None);
        assert_eq!(board.get_piece_debug("g5"), None);
        assert_eq!(board.get_piece_debug("h5"), None);

        // Rank 6
        assert_eq!(board.get_piece_debug("a6"), None);
        assert_eq!(board.get_piece_debug("b6"), None);
        assert_eq!(board.get_piece_debug("c6"), None);
        assert_eq!(board.get_piece_debug("d6"), None);

        assert_eq!(board.get_piece_debug("e6"), None);
        assert_eq!(board.get_piece_debug("f6"), None);
        assert_eq!(board.get_piece_debug("g6"), None);
        assert_eq!(board.get_piece_debug("h6"), None);

        // Rank 7
        assert_eq!(board.get_piece_debug("a7"), Some(PlayerPiece::new(Player::Black, Piece::Pawn)));
        assert_eq!(board.get_piece_debug("b7"), Some(PlayerPiece::new(Player::Black, Piece::Pawn)));
        assert_eq!(board.get_piece_debug("c7"), Some(PlayerPiece::new(Player::Black, Piece::Pawn)));
        assert_eq!(board.get_piece_debug("d7"), Some(PlayerPiece::new(Player::Black, Piece::Pawn)));

        assert_eq!(board.get_piece_debug("e7"), Some(PlayerPiece::new(Player::Black, Piece::Pawn)));
        assert_eq!(board.get_piece_debug("f7"), Some(PlayerPiece::new(Player::Black, Piece::Pawn)));
        assert_eq!(board.get_piece_debug("g7"), Some(PlayerPiece::new(Player::Black, Piece::Pawn)));
        assert_eq!(board.get_piece_debug("h7"), Some(PlayerPiece::new(Player::Black, Piece::Pawn)));

        // Rank 8
        assert_eq!(board.get_piece_debug("a8"), Some(PlayerPiece::new(Player::Black, Piece::Rook)));
        assert_eq!(board.get_piece_debug("b8"), Some(PlayerPiece::new(Player::Black, Piece::Knight)));
        assert_eq!(board.get_piece_debug("c8"), Some(PlayerPiece::new(Player::Black, Piece::Bishop)));
        assert_eq!(board.get_piece_debug("d8"), Some(PlayerPiece::new(Player::Black, Piece::Queen)));

        assert_eq!(board.get_piece_debug("e8"), Some(PlayerPiece::new(Player::Black, Piece::King)));
        assert_eq!(board.get_piece_debug("f8"), Some(PlayerPiece::new(Player::Black, Piece::Bishop)));
        assert_eq!(board.get_piece_debug("g8"), Some(PlayerPiece::new(Player::Black, Piece::Knight)));
        assert_eq!(board.get_piece_debug("h8"), Some(PlayerPiece::new(Player::Black, Piece::Rook)));
    }
}
