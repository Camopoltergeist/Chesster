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

		let mut max = Evaluation::Initial;

		for m in legal_moves {
			let mut moved_position = position.clone();
			moved_position.make_move(m).unwrap();

			let score = -negamax(&moved_position, evaluation_fn, depth - 1);

			if score > max {
				max = score;
			}
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

pub fn negamax_with_move_chain(position: &Position, evaluation_fn: EvaluationFn, depth: u32) -> (Evaluation, Vec<Move>) {
	fn negamax(position: &Position, evaluation_fn: EvaluationFn, depth: u32) -> (Evaluation, Vec<Move>) {
		if depth < 1 {
			return (evaluation_fn(position), Vec::new());
		};

		let legal_moves = position.get_all_legal_moves();

		if legal_moves.len() < 1 {
			return (evaluation_fn(position), Vec::new());
		}

		let mut best_eval = Evaluation::Initial;
		let mut best_move_chain = Vec::new();

		for m in legal_moves {
			let mut moved_position = position.clone();
			moved_position.make_move(m.clone()).unwrap();

			let (mut eval, mut eval_move_chain) = negamax(&moved_position, evaluation_fn, depth - 1);
			eval = -eval;

			eval_move_chain.push(m);

			if eval > best_eval {
				best_eval = eval;
				best_move_chain = eval_move_chain;
			}
		};

		return (best_eval, best_move_chain);
	}

	if depth < 1 {
		return (evaluation_fn(position), Vec::new());
	};

	let legal_moves = position.get_all_legal_moves();

	if legal_moves.len() < 1 {
		return (evaluation_fn(position), Vec::new());
	};

	let mut best_eval = Evaluation::Initial;
	let mut best_move_chain = Vec::new();

	for m in legal_moves {
		let mut moved_position = position.clone();
		moved_position.make_move(m.clone()).unwrap();

		let (mut eval, mut eval_move_chain) = negamax(&moved_position, evaluation_fn, depth - 1);
		eval = -eval;
		
		eval_move_chain.push(m);

		print_move_chain(&eval_move_chain, eval.clone());

		if eval > best_eval {
			best_eval = eval.clone();
			best_move_chain = eval_move_chain;
		}
	};

	return (best_eval, best_move_chain);
}

pub fn print_move_chain(move_chain: &Vec<Move>, evaluation: Evaluation) {
	for m in move_chain.iter().rev() {
		print!("{} | ", m.debug_string());
	}

	println!("{:?}", evaluation);
}