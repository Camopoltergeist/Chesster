use crate::{board::{moove::Move, position::Position}, evaluation::Evaluation};

pub type EvaluationFn = fn(&Position) -> Evaluation;
pub type SearchFn = fn(&Position, EvaluationFn, u32) -> (Move, Evaluation);

pub fn evaluate_material_only(position: &Position) -> Evaluation {
	let own_material = position.board().get_material_for_for_player(position.current_player()) as f32;
	let opponent_material = position.board().get_material_for_for_player(position.current_player().opposite()) as f32;

	Evaluation::Score(own_material - opponent_material)
}

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