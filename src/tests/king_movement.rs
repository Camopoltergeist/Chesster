use crate::{board::{moove::Move, position::Position, tile_position::TilePosition}, tests::compare_moves};

#[test]
fn king_movement_without_collision_a1() {
    let position = Position::from_fen_str("8/8/8/8/8/8/8/K7 w - - 0 1").unwrap();

    let desired_moves = vec![
        Move::debug_new_basic("a1", "a2"),
        Move::debug_new_basic("a1", "b2"),
        Move::debug_new_basic("a1", "b1"),
    ];

    let received_moves = position.get_legal_moves_for_tile_position(TilePosition::from_tile_str("a1").unwrap());

    assert!(compare_moves(&desired_moves, &received_moves)); 
}