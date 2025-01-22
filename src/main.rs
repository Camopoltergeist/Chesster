use board::{bitboard::Bitboard, board::Board};
use ui::start_ui;

mod ui;
pub mod player;
pub mod board;

fn main() {
    let mut board = Board::default();
    board.bishops.print_bitboard();
    board.white_pieces.print_bitboard();
    Board::move_piece(&mut board, player::Player::White, board::piece::Piece::Bishop, 2, 18);
    board.bishops.print_bitboard();
    board.white_pieces.print_bitboard();
    
}