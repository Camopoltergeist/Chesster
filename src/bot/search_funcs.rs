use crate::{board::{moove::Move, position::Position}, bot::evaluation::Evaluation};

use super::EvaluationFn;

pub fn negamax_search(position: &Position, evaluation_fn: EvaluationFn, depth: u32) -> (Move, Evaluation) {
	fn negamax(position: &Position, evaluation_fn: EvaluationFn, depth: u32) -> Evaluation {
		if depth < 1 {
			return evaluation_fn(position);
		};

		let legal_moves = position.get_all_legal_moves();

		if legal_moves.len() < 1 {
			return evaluation_fn(position);
		}

		let mut max = Evaluation::Checkmate(i32::MIN);

		for m in legal_moves {
			let mut moved_position = position.clone();
			moved_position.make_move(m).unwrap();

			let score = -negamax(&moved_position, evaluation_fn, depth - 1);

			if score > max {
				max = score;
			}
		}

		if let Evaluation::Checkmate(moves) = max {
			max = Evaluation::Checkmate(moves + 1);
		}

		return max;
	}

	let legal_moves = position.get_all_legal_moves();

	let mut max = Evaluation::Score(f32::NEG_INFINITY);
	let mut best_move: Option<Move> = None;

	for m in legal_moves {
		let mut moved_position = position.clone();
		moved_position.make_move(m.clone()).unwrap();

		let score = -negamax(&moved_position, evaluation_fn, depth - 1);

		if score > max {
			max = score.clone();
			best_move = Some(m.clone());
		}

		println!("{}: {:?}", m.debug_string(), score);
	}

	return (best_move.unwrap(), max);
}