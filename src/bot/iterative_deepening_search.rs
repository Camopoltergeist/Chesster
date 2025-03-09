use std::{ptr, sync::{mpsc::{self, Receiver, Sender}, Arc}, thread::{self, JoinHandle}, time::{Duration, Instant}};

use crate::{board::{moove::Move, position::Position}, bot::transposition_table::Transposition};

use super::{evaluation_funcs::evaluate_material_and_positioning, transposition_table::TranspositionTable, Bot};

#[derive(Clone)]
pub struct IterativeDeepeningSearch {
	transposition_table: Arc<TranspositionTable>
}

impl IterativeDeepeningSearch {
	pub fn new() -> Self {
		Self {
			transposition_table: Arc::new(TranspositionTable::new())
		}
	}
}

impl Bot for IterativeDeepeningSearch {
	fn search_best_move(&self, position: &Position, search_time: Duration) -> (i32, Move) {
		iterative_deepening(position, evaluate_material_and_positioning, search_time, self.transposition_table.clone())
	}
}

enum SearchMessage {
	FinishedIteration,
	Timeout,
	Continue
}

struct ThreadState {
	pub moove: Move,
	pub join_handle: JoinHandle<i32>,
	pub sender: Sender<SearchMessage>,
	pub receiver: Receiver<SearchMessage>,
	pub finished_iteration: bool,
}

pub fn iterative_deepening(position: &Position, evaluation_fn: fn(&Position) -> i32, search_time: Duration, transposition_table: Arc<TranspositionTable>) -> (i32, Move) {
	let end_time = Instant::now() + search_time;

	// This is converted to usize so it can be sent into a thread
	let tp_ptr_usize = ptr::from_ref(transposition_table.as_ref()) as usize;

	let mut threads: Vec<ThreadState> = Vec::new();

	// Create threads and ThreadState objects
	for m in position.get_all_legal_moves() {
		let (thread_sender, receiver) = mpsc::channel();
		let (sender, thread_receiver) = mpsc::channel();

		let mut moved_position = position.clone();
		moved_position.make_move(m.clone());

		let join_handle = thread::spawn(move || {
			iterative_deepening_thread(moved_position, evaluation_fn, thread_sender, thread_receiver, tp_ptr_usize as *mut TranspositionTable)
		});

		threads.push(ThreadState {
			moove: m,
			join_handle,
			sender,
			receiver,
			finished_iteration: false,
		});
	};

	// Continue search until time runs out.
	// This loop also synchronizes the threads so that any of the threads don't start next iteration while other threads are still calculating current one.
	while Instant::now() < end_time {
		// Read messages from all threads to check for finished iteration
		for t in &mut threads {
			if t.join_handle.is_finished() {
				continue;
			}

			if let Ok(msg) = t.receiver.try_recv() {
				match msg {
					SearchMessage::FinishedIteration => t.finished_iteration = true,
					_ => ()
				}
			}
		}

		// If all threads have finished searching their tree (hit checkmates or stalemates), do an early break
		if threads.iter().all(|t| t.join_handle.is_finished()) {
			break;
		}

		// If all threads have finished current iteration, start next iteration
		if threads.iter().all(|t| t.finished_iteration || t.join_handle.is_finished()) {
			for t in &threads {
				if !t.join_handle.is_finished() {
					_ = t.sender.send(SearchMessage::Continue);
				}
			}
		}
	};

	// Send timeout message to non finished threads
	for t in &threads {
		if !t.join_handle.is_finished() {
			_ = t.sender.send(SearchMessage::Timeout);
		}
	};

	let mut evaluated_moves = Vec::with_capacity(threads.len());

	// Join all threads
	for t in threads {
		let m = t.moove;
		let eval = t.join_handle.join().unwrap();

		evaluated_moves.push((eval, m));
	};

	evaluated_moves.sort_by(|a, b| {
		a.0.cmp(&b.0)
	});

	return evaluated_moves[0].clone();
}

fn iterative_deepening_thread(position: Position, evaluation_fn: fn(&Position) -> i32, sender: Sender<SearchMessage>, receiver: Receiver<SearchMessage>, transposition_table: *mut TranspositionTable) -> i32 {
	let mut depth = 0;

	let mut evaled_moves: Vec<(i32, Move)> = position.get_all_legal_moves().iter().map(|e| (0, e.clone())).collect();
	let mut current_moves = evaled_moves.clone();

	'iterative: loop {
		let alpha = i32::MIN + 1;
		let beta = i32::MAX;

		for (eval_ref, m) in &mut current_moves {
			let mut moved_position = position.clone();
			moved_position.make_move(m.clone());

			let (mut eval, timeout) = alpha_beta(&position, evaluation_fn, -beta, -alpha, depth, &receiver, transposition_table);
			eval = -eval;

			if timeout {
				break 'iterative;
			}

			*eval_ref = eval;
		}

		current_moves.sort_by(|a, b| a.0.cmp(&b.0));
		evaled_moves = current_moves.clone();

		if evaled_moves[0].0 > 100000 {
			break;
		}

		depth += 1;

		sender.send(SearchMessage::FinishedIteration).unwrap();

		let msg = receiver.recv().unwrap();

		if matches!(msg, SearchMessage::Timeout) {
			break;
		}
	};

	return evaled_moves[0].0;
}

fn alpha_beta(position: &Position, evaluation_fn: fn(&Position) -> i32, alpha: i32, beta: i32, depth: u32, receiver: &Receiver<SearchMessage>, transposition_table: *mut TranspositionTable) -> (i32, bool) {
	if depth == 0 {
		return (evaluation_fn(position), false);
	};

	
}

// pub fn iterative_deepening(position: &Position, evaluation_fn: fn(&Position) -> i32, search_time: Duration, transposition_table: Arc<TranspositionTable>) -> (i32, Move) {
// 	let start_time = Instant::now();
// 	let mut depth = 0;

// 	let legal_moves = position.get_all_legal_moves();
// 	let mut evaled_moves: Vec<(i32, Move)> = legal_moves.iter().map(|e| (0, e.clone())).collect();

// 	while Instant::now() - start_time < search_time {
// 		let alpha = i32::MIN + 1;
// 		let beta = i32::MAX;

// 		let mut threads = Vec::new();

// 		let tp_ptr = ptr::from_ref(transposition_table.as_ref()) as *mut TranspositionTable;

// 		for m in &legal_moves {
// 			let mut moved_position = position.clone();
// 			moved_position.make_move(m.clone());

// 			let tp = tp_ptr as usize;
// 			let m = m.clone();

// 			threads.push(thread::spawn(move || {
// 				let tp_ptr = tp as *mut TranspositionTable;

// 				return (-alpha_beta(&moved_position, evaluation_fn, -beta, -alpha, depth, tp_ptr), m);
// 			}));
// 		};

// 		for (i, t) in threads.into_iter().enumerate() {
// 			let (eval, m) = t.join().unwrap();

// 			evaled_moves[i] = (eval, m.clone());
// 		}
		
// 		evaled_moves.sort_by(|a, b| b.0.cmp(&a.0));
// 		depth += 1;

// 		if evaled_moves[0].0.abs() > 100000 {
// 			break;
// 		}
// 	};

// 	for (eval, m) in evaled_moves.iter() {
// 		println!("{} | {}", m.debug_string(), eval);
// 	}

// 	println!("Depth: {}", depth);

// 	return evaled_moves[0].clone();
// }

// fn alpha_beta(position: &Position, evaluation_fn: fn(&Position) -> i32, mut alpha: i32, beta: i32, depth: u32, transposition_table: *mut TranspositionTable) -> i32 {
// 	if depth == 0 {
// 		return evaluation_fn(position);
// 	};

// 	unsafe {
// 		let tp = (*transposition_table).get(position.hash().value());

// 		if tp.hash_matches(position.hash().value()) {
// 			if tp.depth() >= depth {
// 				let eval = tp.evaluation();
// 				if eval.abs() < 1000000 {
// 					return eval;
// 				}
// 			}
// 		}
// 	}

// 	let legal_moves = position.get_all_legal_moves();

// 	if legal_moves.len() == 0 {
// 		if position.is_in_check(position.current_player()) {
// 			return -1000000 * (depth as i32 + 1);
// 		};
		
// 		return 0;
// 	};

// 	for m in legal_moves {
// 		let mut moved_position = position.clone();
// 		moved_position.make_move(m);

// 		let eval = -alpha_beta(&moved_position, evaluation_fn, -beta, -alpha, depth - 1, transposition_table);

// 		let hash = position.hash().value();
// 		// Transposition::new(hash, depth - 1, eval)
// 		unsafe { *(*transposition_table).get(hash) = Transposition::new(hash, depth - 1, eval) };

// 		if eval >= beta {
// 			return eval;
// 		}

// 		alpha = eval.max(alpha);
// 	};

// 	return alpha;
// }