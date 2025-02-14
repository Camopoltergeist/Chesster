use crate::board::{moove::Move, position::Position, tile_position::TilePosition};

use super::compare_moves;

#[test]
fn rook_movement_without_collision_a1() {
    let position = Position::from_fen_str("8/8/8/8/8/8/8/R7 w - - 1 1").unwrap();
    let desired_moves = vec![
        //Moving to the right
        Move::debug_new_basic("a1", "a2"),
        Move::debug_new_basic("a1", "a3"),
        Move::debug_new_basic("a1", "a4"),
        Move::debug_new_basic("a1", "a5"),
        Move::debug_new_basic("a1", "a6"),
        Move::debug_new_basic("a1", "a7"),
        Move::debug_new_basic("a1", "a8"),

        //Moving up
        Move::debug_new_basic("a1", "b1"),
        Move::debug_new_basic("a1", "c1"),
        Move::debug_new_basic("a1", "d1"),
        Move::debug_new_basic("a1", "e1"),
        Move::debug_new_basic("a1", "f1"),
        Move::debug_new_basic("a1", "g1"),
        Move::debug_new_basic("a1", "h1"),
    ];

    let received_moves = position.generate_legal_moves_for_tile_position(TilePosition::from_tile_str("a1").unwrap());
    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn rook_movement_without_collision_h8() {
    let position = Position::from_fen_str("7R/8/8/8/8/8/8/8 w - - 1 1").unwrap();
    let desired_moves = vec![
        //Moving to the left from upper left corner
        Move::debug_new_basic("h8", "a8"),
        Move::debug_new_basic("h8", "b8"),
        Move::debug_new_basic("h8", "c8"),
        Move::debug_new_basic("h8", "d8"),
        Move::debug_new_basic("h8", "e8"),
        Move::debug_new_basic("h8", "f8"),
        Move::debug_new_basic("h8", "g8"),

        //Moving down
        Move::debug_new_basic("h8", "h7"),
        Move::debug_new_basic("h8", "h6"),
        Move::debug_new_basic("h8", "h5"),
        Move::debug_new_basic("h8", "h4"),
        Move::debug_new_basic("h8", "h3"),
        Move::debug_new_basic("h8", "h2"),
        Move::debug_new_basic("h8", "h1"),
    ];

    let received_moves = position.generate_legal_moves_for_tile_position(TilePosition::from_tile_str("h8").unwrap());
    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn rook_collision_with_friendly_pieces() {
    let position = Position::from_fen_str("8/3P4/8/1P1R2PP/8/3P4/3P4/8 w - - 0 1").unwrap();
    let desired_moves = vec![
        //Moving left
        Move::debug_new_basic("d5", "c5"),

        //Moving up
        Move::debug_new_basic("d5", "d6"),

        //Moving down
        Move::debug_new_basic("d5", "d4"),

        //Moving right
        Move::debug_new_basic("d5", "e5"),
        Move::debug_new_basic("d5", "f5"),
    ];

    let received_moves = position.generate_legal_moves_for_tile_position(TilePosition::from_tile_str("d5").unwrap());
    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn rook_collision_with_opponent_pieces() {
    let position = Position::from_fen_str("8/3p4/8/1p1R2pp/8/3p4/3p4/8 w - - 0 1").unwrap();
    let desired_moves = vec![
        //Moving left
        Move::debug_new_basic("d5", "c5"),
        Move::debug_new_basic("d5", "b5"),

        //Moving up
        Move::debug_new_basic("d5", "d6"),
        Move::debug_new_basic("d5", "d7"),

        //Moving down
        Move::debug_new_basic("d5", "d4"),
        Move::debug_new_basic("d5", "d3"),

        //Moving right
        Move::debug_new_basic("d5", "e5"),
        Move::debug_new_basic("d5", "f5"),
        Move::debug_new_basic("d5", "g5"),
    ];

    let received_moves = position.generate_legal_moves_for_tile_position(TilePosition::from_tile_str("d5").unwrap());
    assert!(compare_moves(&desired_moves, &received_moves));
}