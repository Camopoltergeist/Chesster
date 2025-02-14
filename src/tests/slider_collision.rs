use crate::{board::{position::Position, tile_position::TilePosition}, pieces::queen::Queen};

#[test]
fn queen_mask_detects_no_collisions() {
    let position = Position::from_fen_str("P7/4P3/8/P4P2/3Q4/7P/2P1P3/8 w - - 0 1").unwrap();
    let queen_tile_position = TilePosition::from_tile_str("d4").unwrap();

    // hand-written mask of queen's movement from d4 without any collisions
    let desired_move_mask = 0b1000_1000_0100_1001_0010_1010_0001_1100_1111_0111_0001_1100_0010_1010_0100_1001;
    let moves_after_collision = Queen::generate_collision_mask(position.board(), position.current_player(), queen_tile_position);

    assert!(moves_after_collision == desired_move_mask)
}
 #[test]
fn queen_mask_detects_collision_in_ne() {
    let position = Position::from_fen_str("P7/4P1P1/8/P4P2/3Q4/7P/2P1P3/8 w - - 0 1").unwrap();
    let queen_tile_position = TilePosition::from_tile_str("d4").unwrap();

    // hand-written mask of queen's movement from d4 with friendly collision in g7
    let desired_move_mask = 0b100000001001001010100001110011110111000111000010101001001001;
    let moves_after_collision = Queen::generate_collision_mask(position.board(), position.current_player(), queen_tile_position);

    assert!(moves_after_collision == desired_move_mask)
}

#[test]
fn queen_mask_detects_collision_in_e() {
    let position = Position::from_fen_str("P7/4P3/8/P4P2/3Q1P2/7P/2P1P3/8 w - - 0 1").unwrap();
    let queen_tile_position = TilePosition::from_tile_str("d4").unwrap();

    // hand-written mask of queen's movement from d4 with friendly collision in f4
    let desired_move_mask = 0b1000_1000_0100_1001_0010_1010_0001_1100_0001_0111_0001_1100_0010_1010_0100_1001;
    let moves_after_collision = Queen::generate_collision_mask(position.board(), position.current_player(), queen_tile_position);

    assert!(moves_after_collision == desired_move_mask)
}

#[test]
fn queen_mask_detects_collision_in_se() {
    let position = Position::from_fen_str("P7/4P3/8/P4P2/3Q4/4P2P/2P1P3/8 w - - 0 1").unwrap();
    let queen_tile_position = TilePosition::from_tile_str("d4").unwrap();

    // hand-written mask of queen's movement from d4 with friendly collision in g3
    let desired_move_mask = 0b1000100001001001001010100001110011110111000011000000101000001001;
    let moves_after_collision = Queen::generate_collision_mask(position.board(), position.current_player(), queen_tile_position);

    assert!(moves_after_collision == desired_move_mask)
}

#[test]
fn queen_mask_detects_collision_in_s() {
    let position = Position::from_fen_str("P7/4P3/8/P4P2/3Q4/3P3P/2P1P3/8 w - - 0 1").unwrap();
    let queen_tile_position = TilePosition::from_tile_str("d4").unwrap();

    // hand-written mask of queen's movement from d4 with friendly collision in d3
    let desired_move_mask = 0b1000100001001001001010100001110011110111000101000010001001000001;
    let moves_after_collision = Queen::generate_collision_mask(position.board(), position.current_player(), queen_tile_position);

    assert!(moves_after_collision == desired_move_mask)
}

#[test]
fn queen_mask_detects_collision_in_sw() {
    let position = Position::from_fen_str("P7/4P3/8/P4P2/3Q4/7P/2P1P3/P7 w - - 0 1").unwrap();
    let queen_tile_position = TilePosition::from_tile_str("d4").unwrap();

    // hand-written mask of queen's movement from d4 with friendly collision in a1
    let desired_move_mask = 0b1000100001001001001010100001110011110111000111000010101001001000;
    let moves_after_collision = Queen::generate_collision_mask(position.board(), position.current_player(), queen_tile_position);

    assert!(moves_after_collision == desired_move_mask)
}

#[test]
fn queen_mask_detects_collision_in_w() {
    let position = Position::from_fen_str("1P6/3P4/8/P5P1/1P2Q3/7P/3P4/8 w - - 0 1").unwrap();
    let queen_tile_position = TilePosition::from_tile_str("e4").unwrap();

    // hand-written mask of queen's movement from e4 with friendly collision in b4
    let desired_move_mask = 0b0001000110010010010101000011100011101100001110000101010010010010;
    let moves_after_collision = Queen::generate_collision_mask(position.board(), position.current_player(), queen_tile_position);

    assert!(moves_after_collision == desired_move_mask)
}

#[test]
fn queen_mask_detects_collision_in_nw() {
    let position = Position::from_fen_str("PP6/3P4/8/P5P1/4Q3/7P/3P4/8 w - - 0 1").unwrap();
    let queen_tile_position = TilePosition::from_tile_str("e4").unwrap();

    // hand-written mask of queen's movement from e4 with friendly collision in a8
    let desired_move_mask = 0b0001000010010010010101000011100011101111001110000101010010010010;
    let moves_after_collision = Queen::generate_collision_mask(position.board(), position.current_player(), queen_tile_position);

    assert!(moves_after_collision == desired_move_mask)
}

#[test]
fn queen_mask_detects_collision_in_n() {
    let position = Position::from_fen_str("1P6/3P4/8/P3P1P1/4Q3/7P/3P4/8 w - - 0 1").unwrap();
    let queen_tile_position = TilePosition::from_tile_str("e4").unwrap();

    // hand-written mask of queen's movement from e4 with friendly collision in b5
    let desired_move_mask = 0b000110000010010001000010100011101111001110000101010010010010;
    let moves_after_collision = Queen::generate_collision_mask(position.board(), position.current_player(), queen_tile_position);

    assert!(moves_after_collision == desired_move_mask)
}