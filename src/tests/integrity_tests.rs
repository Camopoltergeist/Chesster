use crate::{board::position::Position, bot::{evaluation_funcs::evaluate_material_and_mobility, search_funcs::negamax_with_move_chain}};


/// Position 5 in https://www.chessprogramming.org/Perft_Results
#[test]
fn integrity_test_depth_1() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let (_, _, positions_searched) = negamax_with_move_chain(&position, evaluate_material_and_mobility, 1);
    assert_eq!(positions_searched, 44);
}

#[test]
fn integrity_test_depth_2() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let (_, _, positions_searched) = negamax_with_move_chain(&position, evaluate_material_and_mobility, 2);
    assert_eq!(positions_searched, 1486);
}

// #[test]
pub fn integrity_test_depth_3() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let (_, _, positions_searched) = negamax_with_move_chain(&position, evaluate_material_and_mobility, 3);
    assert_eq!(positions_searched, 62379);
}

// #[ignore]
// #[test]
pub fn integrity_test_depth_4() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let (_, _, positions_searched) = negamax_with_move_chain(&position, evaluate_material_and_mobility, 4);
    assert_eq!(positions_searched, 2103487);
}