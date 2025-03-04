use std::{collections::HashMap, sync::{Arc, Mutex, RwLock}, thread, time::{Duration, Instant}};

use crate::{board::{moove::Move, position::Position}, bot::{evaluation::Evaluation, transposition_table::Transposition}};

use super::{transposition_table::TranspositionTable, EvaluationFn};

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
			moved_position.make_move(m);

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
		moved_position.make_move(m.clone());

		let score = -negamax(&moved_position, evaluation_fn, depth - 1);

		if score > max {
			max = score.clone();
			best_move = Some(m.clone());
		}

		println!("{}: {:?}", m.debug_string(), score);
	}

	return (best_move.unwrap(), max);
}

pub fn negamax_with_move_chain(position: &Position, evaluation_fn: EvaluationFn, depth: u32) -> (Evaluation, Vec<Move>, u64) {
	fn negamax(position: &Position, evaluation_fn: EvaluationFn, depth: u32, searched_positions: &mut u64) -> (Evaluation, Vec<Move>) {
		if depth < 1 {
			*searched_positions += 1;
			return (evaluation_fn(position), Vec::new());
		};

		let legal_moves = position.get_all_legal_moves();

		if legal_moves.len() < 1 {
			*searched_positions += 1;
			return (evaluation_fn(position), Vec::new());
		}

		let mut best_eval = Evaluation::Initial;
		let mut best_move_chain = Vec::new();

		for m in legal_moves {
			let mut moved_position = position.clone();
			moved_position.make_move(m.clone());

			let (mut eval, mut eval_move_chain) = negamax(&moved_position, evaluation_fn, depth - 1, searched_positions);
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
		return (evaluation_fn(position), Vec::new(), 0);
	};

	let legal_moves = position.get_all_legal_moves();

	if legal_moves.len() < 1 {
		return (evaluation_fn(position), Vec::new(), 0);
	};

	let mut best_eval = Evaluation::Initial;
	let mut best_move_chain = Vec::new();

	let mut total_searched = 0; 

	
	for m in legal_moves {
		let mut moved_position = position.clone();
		moved_position.make_move(m.clone());
		
		let mut searched_positions = 0;
		let (mut eval, mut eval_move_chain) = negamax(&moved_position, evaluation_fn, depth - 1, &mut searched_positions);
		eval = -eval;
		
		eval_move_chain.push(m);

		print_move_chain(&eval_move_chain, eval.clone());
		println!("Searched {}", searched_positions);
		total_searched += searched_positions;

		if eval > best_eval {
			best_eval = eval.clone();
			best_move_chain = eval_move_chain;
		}
	};

	return (best_eval, best_move_chain, total_searched);
}

pub fn print_move_chain(move_chain: &Vec<Move>, evaluation: Evaluation) {
	for m in move_chain.iter().rev() {
		print!("{} | ", m.debug_string());
	}

	println!("{:?}", evaluation);
}

pub fn negamax_with_move_chain_multithreaded(position: &Position, evaluation_fn: EvaluationFn, depth: u32) -> (Evaluation, Vec<Move>) {
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
			moved_position.make_move(m.clone());

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

	let mut threads = Vec::new();

	for m in legal_moves {
		let mut moved_position = position.clone();
		moved_position.make_move(m.clone());

		threads.push(thread::spawn(move || {

			let (mut eval, mut eval_move_chain) = negamax(&moved_position, evaluation_fn, depth - 1);
			eval = -eval;

			eval_move_chain.push(m.clone());

			return (eval, eval_move_chain);
		}));
	};

	for t in threads {
		let (eval, eval_move_chain) = t.join().unwrap();

		print_move_chain(&eval_move_chain, eval.clone());

		if eval > best_eval {
			best_eval = eval.clone();
			best_move_chain = eval_move_chain;
		}
	}

	return (best_eval, best_move_chain);
}

pub fn negamax_with_position_cache_multithreaded(position: &Position, evaluation_fn: EvaluationFn, depth: u32, move_cache: Arc<Mutex<HashMap<(u32, Position), Evaluation>>>) -> (Evaluation, Move) {
	fn negamax(position: &Position, evaluation_fn: EvaluationFn, depth: u32, move_cache: Arc<Mutex<HashMap<(u32, Position), Evaluation>>>) -> Evaluation {
		if depth < 1 {
			return evaluation_fn(position);
		};

		let legal_moves = position.get_all_legal_moves();

		if legal_moves.len() < 1 {
			return evaluation_fn(position);
		}

		let mut best_eval = Evaluation::Initial;

		for m in legal_moves {
			let mut moved_position = position.clone();
			moved_position.make_move(m.clone());

			if let Some(cached_eval) = move_cache.lock().unwrap().get(&(depth, moved_position.clone())) {
				if *cached_eval > best_eval {
					best_eval = cached_eval.clone();
					continue;
				}
			}

			let eval = -negamax(&moved_position, evaluation_fn, depth - 1, move_cache.clone());

			if eval > best_eval {
				best_eval = eval.clone();
			}

			move_cache.lock().unwrap().insert((depth, moved_position), eval);
		};

		return best_eval;
	}

	let legal_moves = position.get_all_legal_moves();

	let mut best_eval = Evaluation::Initial;
	let mut best_move: Option<Move> = None;

	let mut threads = Vec::new();

	for m in legal_moves {
		let mut moved_position = position.clone();
		moved_position.make_move(m.clone());

		let mc = move_cache.clone();

		threads.push(thread::spawn(move || {
			let eval = -negamax(&moved_position, evaluation_fn, depth - 1, mc);

			return (eval, m);
		}));
	};

	for t in threads {
		let (eval, moove) = t.join().unwrap();

		if eval > best_eval {
			best_eval = eval.clone();
			best_move = Some(moove.clone());
		}

		println!("{}: {:?}", moove.debug_string(), eval);
	}

	return (best_eval, best_move.unwrap());
}

pub fn alpha_beta_search(position: &Position, evaluation_fn: fn(&Position) -> i32, depth: u32) -> (i32, Move) {
	fn alpha_beta(position: &Position, evaluation_fn: fn(&Position) -> i32, mut alpha: i32, beta: i32, depth: u32) -> i32 {
		if depth == 0 {
			return evaluation_fn(position);
		};

		let legal_moves = position.get_all_legal_moves();

		if legal_moves.len() == 0 {
			if position.is_in_check(position.current_player()) {
				return -1000000 * (depth as i32 + 1);
			};
			
			return 0;
		};

		for m in legal_moves {
			let mut moved_position = position.clone();
			moved_position.make_move(m);

			let eval = -alpha_beta(&moved_position, evaluation_fn, -beta, -alpha, depth - 1);

			if eval >= beta {
				return eval;
			}

			alpha = eval.max(alpha);
		};

		return alpha;
	}

	let mut alpha = i32::MIN + 1;
	let beta = i32::MAX;

	let mut best_move: Option<Move> = None; 

	for m in position.get_all_legal_moves() {
		let mut moved_position = position.clone();
		moved_position.make_move(m.clone());

		let eval = -alpha_beta(&moved_position, evaluation_fn, -beta, -alpha, depth - 1);

		if eval > alpha {
			alpha = eval;
			best_move = Some(m);
		}
	};

	return (alpha, best_move.unwrap());
}

pub fn alpha_beta_search_multithreaded(position: &Position, evaluation_fn: fn(&Position) -> i32, depth: u32) -> (i32, Move) {
	fn alpha_beta(position: &Position, evaluation_fn: fn(&Position) -> i32, mut alpha: i32, beta: i32, depth: u32) -> i32 {
		if depth == 0 {
			return evaluation_fn(position);
		};

		let legal_moves = position.get_all_legal_moves();

		if legal_moves.len() == 0 {
			if position.is_in_check(position.current_player()) {
				return -1000000 * (depth as i32 + 1);
			};

			return 0;
		};

		for m in legal_moves {
			let mut moved_position = position.clone();
			moved_position.make_move(m);

			let eval = -alpha_beta(&moved_position, evaluation_fn, -beta, -alpha, depth - 1);

			if eval >= beta {
				return eval;
			}

			alpha = eval.max(alpha);
		};

		return alpha;
	}

	let alpha = i32::MIN + 1;
	let beta = i32::MAX;

	let mut threads = Vec::new();
	let legal_moves = position.get_all_legal_moves();
	
	for m in &legal_moves {
		let mut moved_position = position.clone();
		moved_position.make_move(m.clone());
		
		threads.push(thread::spawn(move || {
			return -alpha_beta(&moved_position, evaluation_fn, -beta, -alpha, depth - 1);
		}));
	};
	
	let mut best_move: Option<Move> = None; 
	let mut best_eval = i32::MIN;

	for (i, thread) in threads.into_iter().enumerate() {
		let eval = thread.join().unwrap();

		if eval > best_eval {
			best_eval = eval;
			best_move = Some(legal_moves[i].clone());
		}
	}

	return (best_eval, best_move.unwrap());
}

pub fn iterative_deepening(position: &Position, evaluation_fn: fn(&Position) -> i32, search_time: Duration, transposition_table: Arc<TranspositionTable>) -> (i32, Move) {
	fn alpha_beta(position: &Position, evaluation_fn: fn(&Position) -> i32, mut alpha: i32, beta: i32, depth: u32, transposition_table: Arc<TranspositionTable>) -> i32 {
		if depth == 0 {
			return evaluation_fn(position);
		};

		let tp_mutex = transposition_table.get(position.hash().value());

		if let Ok(tp_opt) = tp_mutex.lock() {
			if let Some(tp) = tp_opt.clone() {
				if tp.depth() >= depth as u16 {
					if tp.evaluation().abs() < 1000000 {
						return tp.evaluation();
					}
				}
		}

		}

		let legal_moves = position.get_all_legal_moves();

		if legal_moves.len() == 0 {
			if position.is_in_check(position.current_player()) {
				return -1000000 * (depth as i32 + 1);
			};
			
			return 0;
		};

		for m in legal_moves {
			let mut moved_position = position.clone();
			moved_position.make_move(m);

			let eval = -alpha_beta(&moved_position, evaluation_fn, -beta, -alpha, depth - 1, transposition_table.clone());

			let tp_mutex = transposition_table.get(position.hash().value());

			if let Ok(mut tp_opt) = tp_mutex.lock() {
				*tp_opt = Some(Transposition::new(depth as u16 - 1, eval));
			}

			if eval >= beta {
				return eval;
			}

			alpha = eval.max(alpha);
		};

		return alpha;
	}

	let start_time = Instant::now();
	let mut depth = 0;

	let legal_moves = position.get_all_legal_moves();
	let mut evaled_moves: Vec<(i32, Move)> = legal_moves.iter().map(|e| (0, e.clone())).collect();

	while Instant::now() - start_time < search_time {
		let alpha = i32::MIN + 1;
		let beta = i32::MAX;

		let mut threads = Vec::new();

		for m in &legal_moves {
			let mut moved_position = position.clone();
			moved_position.make_move(m.clone());

			let tp = transposition_table.clone();
			let m = m.clone();

			threads.push(thread::spawn(move || {
				return (-alpha_beta(&moved_position, evaluation_fn, -beta, -alpha, depth, tp), m);
			}));
		};

		for (i, t) in threads.into_iter().enumerate() {
			let (eval, m) = t.join().unwrap();

			evaled_moves[i] = (eval, m.clone());
		}
		
		evaled_moves.sort_by(|a, b| b.0.cmp(&a.0));
		depth += 1;

		if evaled_moves[0].0.abs() > 100000 {
			break;
		}
	};

	for (eval, m) in evaled_moves.iter() {
		println!("{} | {}", m.debug_string(), eval);
	}

	println!("Depth: {}", depth);

	return evaled_moves[0].clone();
}