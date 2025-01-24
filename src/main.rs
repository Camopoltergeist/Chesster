use board::{bitboard::Bitboard, board::Board, move_mask::generate_rook_masks};
use ui::start_ui;

pub mod board;
pub mod player;
mod ui;
pub mod piece;

fn main() {
    generate_rook_masks();
    start_ui();
}
