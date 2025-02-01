use crate::{board::{board::Board, position::Position}, player::Player};

pub fn create_debug_position() -> Position {
	let board = Board::empty();

	let position = Position::new(board, Player::White);

	return position;
}