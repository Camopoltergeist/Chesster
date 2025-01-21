use super::bitboard::Bitboard;

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
            white_pieces: Bitboard(0b_0000000_0000000_0000000_0000000_0000000_00000000_11111111_11111111),
            black_pieces: Bitboard(0b_11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000),
            pawns: Bitboard(0b_00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000),
            rooks: Bitboard(0b_10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001),
            knights: Bitboard(0b_01000010_00000000_00000000_00000000_00000000_00000000_00000000_01000010),
            bishops: Bitboard(0b_00100100_00000000_00000000_00000000_00000000_00000000_00000000_00100100),
            queens: Bitboard(0b_00001000_00000000_00000000_00000000_00000000_00000000_00000000_00001000),
            kings: Bitboard(0b_00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000),
        }
    }
}

impl Board {
    fn check_overlaps(a: Bitboard, b: Bitboard) -> bool {
        a.0 & b.0 != 0
    }
    
    fn validate(&self) -> bool {
        !(
            Board::check_overlaps(self.pawns, self.rooks) ||
            Board::check_overlaps(self.pawns, self.knights) ||
            Board::check_overlaps(self.pawns, self.bishops) ||
            Board::check_overlaps(self.pawns, self.queens) ||
            Board::check_overlaps(self.pawns, self.kings) ||

            Board::check_overlaps(self.rooks, self.knights) ||
            Board::check_overlaps(self.rooks, self.bishops) ||
            Board::check_overlaps(self.rooks, self.queens) ||
            Board::check_overlaps(self.rooks, self.kings) ||

            Board::check_overlaps(self.knights, self.bishops) ||
            Board::check_overlaps(self.knights, self.queens) ||
            Board::check_overlaps(self.knights, self.kings) ||

            Board::check_overlaps(self.bishops, self.queens) ||
            Board::check_overlaps(self.bishops, self.kings) ||

            Board::check_overlaps(self.queens, self.kings)
        )

    }
}