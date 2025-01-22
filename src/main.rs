use board::{bitboard::Bitboard, board::Board};
use ui::start_ui;

pub mod board;
pub mod player;
mod ui;

fn main() {
    let mut board = Board::default();
    board.bishops.print_bitboard();
    board.white_pieces.print_bitboard();
    Board::move_piece(
        &mut board,
        player::Player::White,
        board::piece::Piece::Bishop,
        2,
        18,
    );
    board.bishops.print_bitboard();
    board.white_pieces.print_bitboard();
    let tuple = board.get_piece(62);
    println!("{:?} {:?}", tuple.0, tuple.1);
}
