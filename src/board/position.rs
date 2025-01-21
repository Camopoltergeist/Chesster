use super::board::Board;

pub struct Position {
    board: Board,
    en_passant: bool,
    white_short_castling: bool,
    white_long_castling: bool,
    black_short_castling: bool,
    black_long_castling: bool,
}