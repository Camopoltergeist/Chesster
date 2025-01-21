use crate::board::board::Board;

pub mod board;

fn main() {
    let board = Board::default();


    board.rooks.value();
}
