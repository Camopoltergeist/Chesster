use crate::{board::{moove::Move, position::Position}, piece::PieceType, player::Player};

#[test]
fn en_passant_works() {
    let mut position = Position::from_fen_str("7k/8/8/2Pp4/8/8/8/7K w - d6 0 2").unwrap();

    let en_passant_move = Move::debug_new_en_passant("c5", "d6", "d5");

    position.make_move(en_passant_move).unwrap();

    assert!(position.debug_check_tile("d6", Some((Player::White, PieceType::Pawn))));
    assert!(position.debug_check_tile("d5", None));
}