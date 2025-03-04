use std::{sync::{Arc, RwLock}, time::Duration};

use crate::board::{moove::Move, position::Position};

use super::{evaluation_funcs::evaluate_material_and_positioning, search_funcs::iterative_deepening, transposition_table::TranspositionTable, Bot};

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