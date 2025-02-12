use crate::{board::{moove::{CastleSide, Move}, position::Position}, piece::PieceType, player::Player};

#[test]
fn king_side_castling_move_works() {
	let mut position = Position::from_fen_str("8/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();

	let castling_move = Move::new_castling(Player::White, CastleSide::KingSide);

	position.make_move(castling_move).unwrap();

	assert!(position.debug_check_tile("g1", Some((Player::White, PieceType::King))));
	assert!(position.debug_check_tile("f1", Some((Player::White, PieceType::Rook))));
}

#[test]
fn queen_side_castling_move_works() {
	let mut position = Position::from_fen_str("8/8/8/8/8/8/8/R3K3 w Q - 1 1").unwrap();

	let castling_move = Move::new_castling(Player::White, CastleSide::QueenSide);

	position.make_move(castling_move).unwrap();

	assert!(position.debug_check_tile("c1", Some((Player::White, PieceType::King))));
	assert!(position.debug_check_tile("d1", Some((Player::White, PieceType::Rook))));
}

#[test]
fn cant_castle_while_checked() {
	let position = Position::from_fen_str("4r3/8/8/8/8/8/8/4K2R b K - 2 1").unwrap();

	let castling_move = Move::new_castling(Player::White, CastleSide::KingSide);

	assert!(!position.is_legal_move(&castling_move));
}

#[test]
fn cant_castle_while_path_is_threatened() {
	let position = Position::from_fen_str("5r2/8/8/8/8/8/8/4K2R b K - 2 1").unwrap();

	let castling_move = Move::new_castling(Player::White, CastleSide::KingSide);

	assert!(!position.is_legal_move(&castling_move));
}