use crate::{board::{moove::Move, position::Position}, piece::PieceType, player::Player, player_piece::PlayerPiece};

use super::compare_moves;

#[test]
fn can_promote_to_all_pieces() {
    let position = Position::from_fen_str("8/P7/8/8/8/8/8/8 w - - 0 2").unwrap();

    let desired_moves = vec![
        Move::debug_new_promoting("a7", "a8", PlayerPiece::new(Player::White, PieceType::Queen)),
        Move::debug_new_promoting("a7", "a8", PlayerPiece::new(Player::White, PieceType::Bishop)),
        Move::debug_new_promoting("a7", "a8", PlayerPiece::new(Player::White, PieceType::Knight)),
        Move::debug_new_promoting("a7", "a8", PlayerPiece::new(Player::White, PieceType::Rook)),
    ];

    let received_moves = position.get_all_legal_moves();

    assert!(compare_moves(&desired_moves, &received_moves));
}