use std::{collections::HashMap, sync::Arc, time::Duration};

use rand::seq::IndexedRandom;

use crate::{board::{moove::Move, position::Position}, opening_book::load_opening_book};

use super::{search_funcs::{iterative_deepening, iterative_deepening_no_ext}, transposition_table::TranspositionTable, Bot};

#[derive(Clone)]
pub struct IterativeDeepeningSearch {
	transposition_table: Arc<TranspositionTable>,
	evaluation_fn: fn(&Position) -> i32,
	opening_book: HashMap<u64, Vec<Move>>,
	use_extensions: bool,
}

impl IterativeDeepeningSearch {
	pub fn new(evaluation_fn: fn(&Position) -> i32, use_extensions: bool) -> Self {
		Self {
			transposition_table: Arc::new(TranspositionTable::new()),
			evaluation_fn,
			opening_book: load_opening_book(),
			use_extensions
		}
	}
}

impl Bot for IterativeDeepeningSearch {
	fn search_best_move(&self, position: &Position, search_time: Duration) -> (i32, Move) {
		if let Some(next_moves) = self.opening_book.get(&position.hash().value()) {
			let m_opt = next_moves.choose(&mut rand::rng());

			for m in next_moves {
				println!("B: {}", m.debug_string());
			}
			
			if let Some(m) = m_opt {
				println!("Book move: {}", m.debug_string());
				return (0, m.clone());
			}
		}

		if self.use_extensions {
			iterative_deepening(position, self.evaluation_fn, search_time, self.transposition_table.clone())
		}
		else {
			iterative_deepening_no_ext(position, self.evaluation_fn, search_time, self.transposition_table.clone())
		}
	}
}