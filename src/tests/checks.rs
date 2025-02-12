use crate::board::{moove::Move, position::Position};

#[test]
fn cant_leave_king_in_check() {
	let position = Position::from_fen_str("4r3/8/8/8/8/8/8/4KR2 w - - 0 1").unwrap();

	let illegal_move = Move::debug_new_basic("f1", "f2");

	assert!(!position.is_legal_move(&illegal_move));
}

#[test]
fn cant_move_away_while_pinned() {
	let position = Position::from_fen_str("4r3/8/8/8/8/8/4B3/4K3 w - - 0 1").unwrap();

	let illegal_move = Move::debug_new_basic("e2", "a6");

	assert!(!position.is_legal_move(&illegal_move));
}