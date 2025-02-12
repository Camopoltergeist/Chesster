use crate::{board::{moove::{CastleSide, Move}, position::Position, tile_position::TilePosition}, piece::PieceType, player::Player, player_piece::PlayerPiece};

#[test]
fn king_side_castling_move_works() {
	let mut position = Position::from_fen_str("8/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();

	let castling_move = Move::new_castling(Player::White, CastleSide::KingSide);

	position.make_move(castling_move).unwrap();

	assert_eq!(position.get_piece(TilePosition::from_tile_str("g1").unwrap()), Some(PlayerPiece::new(Player::White, PieceType::King)));
}