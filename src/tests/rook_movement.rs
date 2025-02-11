use crate::board::{moove::{BasicMove, Move}, position::Position, tile_position::TilePosition};

use super::compare_moves;

#[test]
fn rook_movement_without_collision_a1() {
    let position = Position::from_fen_str("8/8/8/8/8/8/8/R7 w - - 1 1").unwrap();
    let desired_moves = vec![
        Move::debug_new_basic("a1", "a2"),
        Move::debug_new_basic("a1", "a3"),
        Move::debug_new_basic("a1", "a4"),
        Move::debug_new_basic("a1", "a5"),
        Move::debug_new_basic("a1", "a6"),
        Move::debug_new_basic("a1", "a7"),
        Move::debug_new_basic("a1", "a8"),
        Move::debug_new_basic("a1", "b1"),
        Move::debug_new_basic("a1", "c1"),
        Move::debug_new_basic("a1", "d1"),
        Move::debug_new_basic("a1", "e1"),
        Move::debug_new_basic("a1", "f1"),
        Move::debug_new_basic("a1", "g1"),
        Move::debug_new_basic("a1", "h1"),
    ];

    let received_moves = position.get_all_legal_moves();
    assert!(compare_moves(&desired_moves, &received_moves));
}
