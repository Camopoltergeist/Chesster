use crate::{board::position::Position, perft::perft};

/// Position 5 in https://www.chessprogramming.org/Perft_Results
#[test]
fn integrity_test_depth_1() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let positions_searched = perft(&position, 1)
        .iter()
        .map(|e| e.1)
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);

    assert_eq!(positions_searched, 44);
}

#[test]
fn integrity_test_depth_2() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let positions_searched = perft(&position, 2)
        .iter()
        .map(|e| e.1)
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);

    assert_eq!(positions_searched, 1486);
}

#[test]
pub fn integrity_test_depth_3() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let positions_searched = perft(&position, 3)
        .iter()
        .map(|e| e.1)
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);

    assert_eq!(positions_searched, 62379);
}

#[ignore]
#[test]
pub fn integrity_test_depth_4() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let positions_searched = perft(&position, 4)
        .iter()
        .map(|e| e.1)
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);

    assert_eq!(positions_searched, 2103487);
}

#[ignore]
#[test]
pub fn integrity_test_depth_5() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let positions_searched = perft(&position, 5)
        .iter()
        .map(|e| e.1)
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);

    assert_eq!(positions_searched, 89941194);
}

#[ignore]
#[test]
pub fn integrity_test_depth_6() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let positions_searched = perft(&position, 6)
        .iter()
        .map(|e| e.1)
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);

    assert_eq!(positions_searched, 3048196529);
}