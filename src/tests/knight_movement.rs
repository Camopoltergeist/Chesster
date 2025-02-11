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

#[test]
fn knight_movement_without_collision_d4() {
    let position = Position::from_fen_str("8/8/8/8/3N4/8/8/8 w - - 0 1").unwrap();

    let desired_moves = vec![
        // North West
        Move::debug_new_basic("d4", "b5"),
        Move::debug_new_basic("d4", "c6"),

        // North East
        Move::debug_new_basic("d4", "e6"),
        Move::debug_new_basic("d4", "f5"),

        // South East
        Move::debug_new_basic("d4", "f3"),
        Move::debug_new_basic("d4", "e2"),

        // South West
        Move::debug_new_basic("d4", "c2"),
        Move::debug_new_basic("d4", "b3"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position(TilePosition::from_tile_str("d4").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));    
}

#[test]
fn knight_collision_with_friendly_pieces() {
    let position = Position::from_fen_str("8/8/2P5/5P2/3N4/1P3P2/8/8 w - - 0 1").unwrap();

    let desired_moves = vec![
        // North West
        Move::debug_new_basic("d4", "b5"),

        // North East
        Move::debug_new_basic("d4", "e6"),

        // South East
        Move::debug_new_basic("d4", "e2"),

        // South West
        Move::debug_new_basic("d4", "c2"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position(TilePosition::from_tile_str("d4").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn knight_collision_with_opponent_pieces() {
    let position = Position::from_fen_str("8/8/2p5/5p2/3N4/1p3p2/8/8 w - - 0 1").unwrap();

    let desired_moves = vec![
        // North West
        Move::debug_new_basic("d4", "b5"),
        Move::debug_new_basic("d4", "c6"),

        // North East
        Move::debug_new_basic("d4", "e6"),
        Move::debug_new_basic("d4", "f5"),

        // South East
        Move::debug_new_basic("d4", "f3"),
        Move::debug_new_basic("d4", "e2"),

        // South West
        Move::debug_new_basic("d4", "c2"),
        Move::debug_new_basic("d4", "b3"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position(TilePosition::from_tile_str("d4").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));    
}