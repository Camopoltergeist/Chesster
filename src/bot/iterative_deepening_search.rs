use std::{sync::Arc, time::Duration};

use crate::board::{moove::Move, position::Position};

use super::{search_funcs::iterative_deepening, transposition_table::TranspositionTable, Bot};

#[derive(Clone)]
pub struct IterativeDeepeningSearch {
	transposition_table: Arc<TranspositionTable>,
	evaluation_fn: fn(&Position) -> i32,
}

impl IterativeDeepeningSearch {
	pub fn new(evaluation_fn: fn(&Position) -> i32) -> Self {
		Self {
			transposition_table: Arc::new(TranspositionTable::new()),
			evaluation_fn
		}
	}
}

impl Bot for IterativeDeepeningSearch {
	fn search_best_move(&self, position: &Position, search_time: Duration) -> (i32, Move) {
		iterative_deepening(position, self.evaluation_fn, search_time, self.transposition_table.clone())
	}
}