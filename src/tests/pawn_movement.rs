use crate::board::{moove::Move, position::Position, tile_position::TilePosition};

use super::compare_moves;

#[test]
fn white_pawn_movement_without_collision_d2() {
    let position = Position::from_fen_str("8/8/8/8/8/8/3P4/8 w - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("d2", "d3"),
        Move::debug_new_basic("d2", "d4"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position_old(TilePosition::from_tile_str("d2").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn black_pawn_movement_without_collision_d7() {
    let position = Position::from_fen_str("8/3p4/8/8/8/8/8/8 b - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("d7", "d6"),
        Move::debug_new_basic("d7", "d5"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position_old(TilePosition::from_tile_str("d7").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn white_pawn_movement_without_collision_d3() {
    let position = Position::from_fen_str("8/8/8/8/8/3P4/8/8 w - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("d3", "d4"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position_old(TilePosition::from_tile_str("d3").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn black_pawn_movement_without_collision_d6() {
    let position = Position::from_fen_str("8/8/3p4/8/8/8/8/8 b - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("d6", "d5"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position_old(TilePosition::from_tile_str("d6").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn white_pawn_collision_with_friendly_pieces_d2d4() {
    let position = Position::from_fen_str("8/8/8/8/3P4/8/3P4/8 w - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("d2", "d3"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position_old(TilePosition::from_tile_str("d2").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn white_pawn_collision_with_opponent_pieces_d2d3() {
    let position = Position::from_fen_str("8/8/8/8/8/3p4/3P4/8 w - - 0 1").unwrap();

    let desired_moves = vec![
    ];

    let received_moves = position.get_legal_moves_for_tile_position_old(TilePosition::from_tile_str("d2").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn black_pawn_collision_with_friendly_pieces_d7d5() {
    let position = Position::from_fen_str("8/3p4/8/3p4/8/8/8/8 b - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("d7", "d6"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position_old(TilePosition::from_tile_str("d7").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn white_pawn_collision_with_opponent_pieces_d7d6() {
    let position = Position::from_fen_str("8/3p4/3P4/8/8/8/8/8 b - - 0 1").unwrap();

    let desired_moves = vec![
    ];

    let received_moves = position.get_legal_moves_for_tile_position_old(TilePosition::from_tile_str("d7").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn white_pawn_capture() {
    let position = Position::from_fen_str("8/8/8/2p1p3/3P4/8/8/8 w - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("d4", "c5"),
        Move::debug_new_basic("d4", "d5"),
        Move::debug_new_basic("d4", "e5"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position_old(TilePosition::from_tile_str("d4").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn black_pawn_capture() {
    let position = Position::from_fen_str("8/8/8/8/3p4/2P1P3/8/8 b - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("d4", "c3"),
        Move::debug_new_basic("d4", "d3"),
        Move::debug_new_basic("d4", "e3"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position_old(TilePosition::from_tile_str("d4").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}
