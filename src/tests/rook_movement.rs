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

    let received_moves = position.get_legal_moves_for_tile_position(TilePosition::from_tile_str("a1").unwrap());
    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn rook_movement_without_collision_h8() {
    let position = Position::from_fen_str("7R/8/8/8/8/8/8/8 w - - 1 1").unwrap();
    let desired_moves = vec![
        Move::debug_new_basic("h8", "a8"),
        Move::debug_new_basic("h8", "b8"),
        Move::debug_new_basic("h8", "c8"),
        Move::debug_new_basic("h8", "d8"),
        Move::debug_new_basic("h8", "e8"),
        Move::debug_new_basic("h8", "f8"),
        Move::debug_new_basic("h8", "g8"),
        Move::debug_new_basic("h8", "h7"),
        Move::debug_new_basic("h8", "h6"),
        Move::debug_new_basic("h8", "h5"),
        Move::debug_new_basic("h8", "h4"),
        Move::debug_new_basic("h8", "h3"),
        Move::debug_new_basic("h8", "h2"),
        Move::debug_new_basic("h8", "h1"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position(TilePosition::from_tile_str("h8").unwrap());
    assert!(compare_moves(&desired_moves, &received_moves));
}