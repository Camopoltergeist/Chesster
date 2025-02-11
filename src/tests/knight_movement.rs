use crate::board::position::Position;
use crate::board::{moove::Move, tile_position::TilePosition};

use crate::tests::compare_moves;

#[test]
fn knight_movement_without_collision_a1() {
    let position = Position::from_fen_str("8/8/8/8/8/8/8/N7 w - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("a1", "b3"),
        Move::debug_new_basic("a1", "c2"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position(TilePosition::from_tile_str("a1").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn knight_movement_without_collision_h8() {
    let position = Position::from_fen_str("7N/8/8/8/8/8/8/8 w - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("h8", "f7"),
        Move::debug_new_basic("h8", "g6"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position(TilePosition::from_tile_str("h8").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));    
}
