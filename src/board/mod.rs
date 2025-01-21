use bitboard::Bitboard;

pub mod bitboard;

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

