use crate::{board::{moove::Move, position::Position}, bot::evaluation::Evaluation};

use super::EvaluationFn;

pub fn negamax_search(position: &Position, evaluation_fn: EvaluationFn, depth: u32) -> (Move, Evaluation) {
	fn negamax(position: &Position, evaluation_fn: EvaluationFn, depth: u32) -> Evaluation {
		if depth == 0 {
			return evaluation_fn(position);
		};

		let mut max = Evaluation::Score(f32::NEG_INFINITY);

		for m in position.get_all_legal_moves() {
			let mut moved_position = position.clone();
			moved_position.make_move(m).unwrap();

			let score = -negamax(&moved_position, evaluation_fn, depth - 1);

			if score > max {
				max = score;
			}
		}

		return max;
	}

	let mut max = Evaluation::Score(f32::NEG_INFINITY);
	let mut best_move: Option<Move> = None;

	for m in position.get_all_legal_moves() {
		let mut moved_position = position.clone();
		moved_position.make_move(m.clone()).unwrap();

		let score = -negamax(&moved_position, evaluation_fn, depth - 1);

		if score > max {
			max = score;
			best_move = Some(m);
		}
	}

	return (best_move.unwrap(), max);
}