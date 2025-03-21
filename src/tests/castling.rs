use crate::{board::{moove::{CastleSide, Move}, position::Position}, piece::PieceType, player::Player};

#[test]
fn king_side_castling_move_works() {
	let mut position = Position::from_fen_str("8/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();

	let castling_move = Move::new_castling(Player::White, CastleSide::KingSide);

	position.make_move(castling_move);

	assert!(position.debug_check_tile("g1", Some((Player::White, PieceType::King))));
	assert!(position.debug_check_tile("f1", Some((Player::White, PieceType::Rook))));
}

#[test]
fn queen_side_castling_move_works() {
	let mut position = Position::from_fen_str("8/8/8/8/8/8/8/R3K3 w Q - 1 1").unwrap();

	let castling_move = Move::new_castling(Player::White, CastleSide::QueenSide);

	position.make_move(castling_move);

	assert!(position.debug_check_tile("c1", Some((Player::White, PieceType::King))));
	assert!(position.debug_check_tile("d1", Some((Player::White, PieceType::Rook))));
}

#[test]
fn cant_castle_queen_side_when_path_is_blocked() {
	let position = Position::from_fen_str("8/8/8/8/8/8/8/RN2K3 w Q - 0 1").unwrap();

	let castling_move = Move::new_castling(Player::White, CastleSide::QueenSide);

	assert!(!position.is_legal_move(&castling_move));
}

#[test]
fn cant_castle_while_checked() {
	let position = Position::from_fen_str("4r3/8/8/8/8/8/8/4K2R w K - 2 1").unwrap();

	let castling_move = Move::new_castling(Player::White, CastleSide::KingSide);

	assert!(!position.is_legal_move(&castling_move));
}

#[test]
fn cant_castle_king_side_while_path_is_threatened() {
	let position = Position::from_fen_str("5r2/8/8/8/8/8/8/4K2R b K - 2 1").unwrap();

	let castling_move = Move::new_castling(Player::White, CastleSide::KingSide);

	assert!(!position.is_legal_move(&castling_move));
}

#[test]
fn can_castle_queen_side_while_b_file_is_threatened() {
	let position = Position::from_fen_str("1r6/8/8/8/8/8/8/R3K3 b Q - 2 1").unwrap();

	let castling_move = Move::new_castling(Player::White, CastleSide::QueenSide);

	assert!(position.is_legal_move(&castling_move));
}

#[test]
fn castling_availability_updates_on_king_move() {
	let mut position = Position::from_fen_str("8/8/8/8/8/8/8/R3K2R w KQ - 2 1").unwrap();

	let king_move = Move::debug_new_basic("e1", "e2");

	position.make_move(king_move);

	assert!(!position.get_castling_availability(Player::White, CastleSide::KingSide));
	assert!(!position.get_castling_availability(Player::White, CastleSide::QueenSide));
}

#[test]
fn castling_availability_updates_on_rook_move() {
	let mut position = Position::from_fen_str("8/8/8/8/8/8/8/R3K2R w KQ - 2 1").unwrap();

	let rook_move = Move::debug_new_basic("a1", "a2");

	position.make_move(rook_move);

	assert!(position.get_castling_availability(Player::White, CastleSide::KingSide));
	assert!(!position.get_castling_availability(Player::White, CastleSide::QueenSide));
}

#[test]
fn castling_availability_updates_on_rook_capture() {
	let mut position = Position::from_fen_str("8/8/8/8/8/8/1b6/R3K2R b KQ - 2 1").unwrap();

	let capturing_move = Move::debug_new_basic("b2", "a1");

	position.make_move(capturing_move);

	assert!(position.get_castling_availability(Player::White, CastleSide::KingSide));
	assert!(!position.get_castling_availability(Player::White, CastleSide::QueenSide));
}