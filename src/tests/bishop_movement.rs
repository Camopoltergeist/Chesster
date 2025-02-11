use crate::{board::{moove::Move, position::Position, tile_position::TilePosition}, tests::compare_moves};

#[test]
fn bishop_movement_without_collision_a1() {
    let position = Position::from_fen_str("8/8/8/8/8/8/8/B7 w - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("a1", "b2"),
        Move::debug_new_basic("a1", "c3"),
        Move::debug_new_basic("a1", "d4"),
        Move::debug_new_basic("a1", "e5"),
        Move::debug_new_basic("a1", "f6"),
        Move::debug_new_basic("a1", "g7"),
        Move::debug_new_basic("a1", "h8"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position(TilePosition::from_tile_str("a1").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}

#[test]
fn bishop_movement_without_collision_d4() {
    let position = Position::from_fen_str("8/8/8/8/3B4/8/8/8 w - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("d4", "a1"),
        Move::debug_new_basic("d4", "b2"),
        Move::debug_new_basic("d4", "c3"),
        Move::debug_new_basic("d4", "e5"),
        Move::debug_new_basic("d4", "f6"),
        Move::debug_new_basic("d4", "g7"),
        Move::debug_new_basic("d4", "h8"),

        Move::debug_new_basic("d4", "a7"),
        Move::debug_new_basic("d4", "b6"),
        Move::debug_new_basic("d4", "c5"),
        Move::debug_new_basic("d4", "e3"),
        Move::debug_new_basic("d4", "f2"),
        Move::debug_new_basic("d4", "g1"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position(TilePosition::from_tile_str("d4").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves));
}