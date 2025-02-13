use crate::{board::{moove::Move, position::Position}, bot::evaluation_funcs::evaluate_material_and_mobility};

fn negamax_perft(position: &Position, depth: u32) -> Vec<(Move, u64)> {
	fn negamax(position: &Position, depth: u32) -> u64 {
		if depth < 1 {
			return 1;
		};

		let legal_moves = position.get_all_legal_moves();

		if legal_moves.len() < 1 {
			return 0;
		}

        let mut searched_positions = 0;

		for m in legal_moves {
			let mut moved_position = position.clone();
			moved_position.make_move(m.clone());

			searched_positions += negamax(&moved_position, depth - 1);
		};

        return searched_positions;
	}

	if depth < 1 {
		return Vec::new();
	};

	let legal_moves = position.get_all_legal_moves();

	if legal_moves.len() < 1 {
		return Vec::new();
	};

	let mut moves = Vec::new();
	
	for m in legal_moves {
		let mut moved_position = position.clone();
		moved_position.make_move(m.clone());
		
		let searched_positions = negamax(&moved_position, depth - 1);
		
        moves.push((m, searched_positions));
	};

	return moves;
}

/// Position 5 in https://www.chessprogramming.org/Perft_Results
#[test]
fn integrity_test_depth_1() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let positions_searched = negamax_perft(&position, 1)
        .iter()
        .map(|e| e.1)
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);

    assert_eq!(positions_searched, 44);
}

#[test]
fn integrity_test_depth_2() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let positions_searched = negamax_perft(&position, 2)
        .iter()
        .map(|e| e.1)
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);

    assert_eq!(positions_searched, 1486);
}

#[test]
pub fn integrity_test_depth_3() {
    let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

    let positions_searched = negamax_perft(&position, 3)
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

    let positions_searched = negamax_perft(&position, 4)
        .iter()
        .map(|e| e.1)
        .reduce(|acc, e| acc + e)
        .unwrap_or(0);
    
    assert_eq!(positions_searched, 2103487);
}