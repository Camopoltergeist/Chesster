use crate::board::{moove::Move, position::Position, tile_position::TilePosition};

use super::compare_moves;

#[test]
fn queen_movement_without_collision_a1() {
    let position = Position::from_fen_str("8/8/8/8/8/8/8/Q7 w - - 0 1").unwrap();
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

        //Moving NorthEast
        Move::debug_new_basic("a1", "b2"),
        Move::debug_new_basic("a1", "c3"),
        Move::debug_new_basic("a1", "d4"),
        Move::debug_new_basic("a1", "e5"),
        Move::debug_new_basic("a1", "f6"),
        Move::debug_new_basic("a1", "g7"),
        Move::debug_new_basic("a1", "h8"),
    ];

    let received_moves = position.generate_legal_moves_for_tile_position(TilePosition::from_tile_str("a1").unwrap());
    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn queen_movement_without_collision_d4() {
    let position = Position::from_fen_str("8/8/8/8/3Q4/8/8/8 w - - 0 1").unwrap();
    let desired_moves = vec![
        //Moving to the left
        Move::debug_new_basic("d4", "c4"),
        Move::debug_new_basic("d4", "b4"),
        Move::debug_new_basic("d4", "a4"),

        //Moving right
        Move::debug_new_basic("d4", "e4"),
        Move::debug_new_basic("d4", "f4"),
        Move::debug_new_basic("d4", "g4"),
        Move::debug_new_basic("d4", "h4"),

        //Moving up
        Move::debug_new_basic("d4", "d5"),
        Move::debug_new_basic("d4", "d6"),
        Move::debug_new_basic("d4", "d7"),
        Move::debug_new_basic("d4", "d8"),

        //Moving down
        Move::debug_new_basic("d4", "d3"),
        Move::debug_new_basic("d4", "d2"),
        Move::debug_new_basic("d4", "d1"),

        // South East
        Move::debug_new_basic("d4", "c3"),
        Move::debug_new_basic("d4", "b2"),
        Move::debug_new_basic("d4", "a1"),

        // North West
        Move::debug_new_basic("d4", "e5"),
        Move::debug_new_basic("d4", "f6"),
        Move::debug_new_basic("d4", "g7"),
        Move::debug_new_basic("d4", "h8"),

        // North East 
        Move::debug_new_basic("d4", "c5"),
        Move::debug_new_basic("d4", "b6"),
        Move::debug_new_basic("d4", "a7"),

        // South West
        Move::debug_new_basic("d4", "e3"),
        Move::debug_new_basic("d4", "f2"),
        Move::debug_new_basic("d4", "g1"),
    ];

    let received_moves = position.generate_legal_moves_for_tile_position(TilePosition::from_tile_str("d4").unwrap());
    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn queen_collision_with_friendly_pieces() {
    let position = Position::from_fen_str("8/3P2P1/5P2/2P5/1P1Q3P/8/1P1P4/6P1 w - - 0 1").unwrap();

    let desired_moves = vec![
        // West
        Move::debug_new_basic("d4", "c4"),

        // North
        Move::debug_new_basic("d4", "d5"),
        Move::debug_new_basic("d4", "d6"),

        // East
        Move::debug_new_basic("d4", "e4"),
        Move::debug_new_basic("d4", "f4"),
        Move::debug_new_basic("d4", "g4"),

        // South
        Move::debug_new_basic("d4", "d3"),

        // North West, no moves

        // South West
        Move::debug_new_basic("d4", "c3"),

        // South East
        Move::debug_new_basic("d4", "e3"),
        Move::debug_new_basic("d4", "f2"),

        // North East
        Move::debug_new_basic("d4", "e5"),
    ];

    let received_moves = position.generate_legal_moves_for_tile_position(TilePosition::from_tile_str("d4").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn queen_collision_with_opponent_pieces() {
    let position = Position::from_fen_str("8/3p2p1/5p2/2p5/1p1Q3p/8/1p1p4/6p1 w - - 0 1").unwrap();

    let desired_moves = vec![
        // West
        Move::debug_new_basic("d4", "c4"),
        Move::debug_new_basic("d4", "b4"),

        // North
        Move::debug_new_basic("d4", "d5"),
        Move::debug_new_basic("d4", "d6"),
        Move::debug_new_basic("d4", "d7"),

        // East
        Move::debug_new_basic("d4", "e4"),
        Move::debug_new_basic("d4", "f4"),
        Move::debug_new_basic("d4", "g4"),
        Move::debug_new_basic("d4", "h4"),

        // South
        Move::debug_new_basic("d4", "d3"),
        Move::debug_new_basic("d4", "d2"),

        // North West
        Move::debug_new_basic("d4", "c5"),

        // South West
        Move::debug_new_basic("d4", "c3"),
        Move::debug_new_basic("d4", "b2"),

        // South East
        Move::debug_new_basic("d4", "e3"),
        Move::debug_new_basic("d4", "f2"),
        Move::debug_new_basic("d4", "g1"),

        // North East
        Move::debug_new_basic("d4", "e5"),
        Move::debug_new_basic("d4", "f6"),
    ];

    let received_moves = position.generate_legal_moves_for_tile_position(TilePosition::from_tile_str("d4").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}