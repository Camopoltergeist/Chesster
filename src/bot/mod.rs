//! Logic for evaluation and search algorithms.

use std::time::Duration;

use dyn_clone::DynClone;
use evaluation::Evaluation;

use crate::board::{moove::Move, position::Position};

pub mod evaluation;
pub mod evaluation_funcs;
pub mod search_funcs;
pub mod positioning;
pub mod utils;
pub mod transposition_table;
pub mod iterative_deepening_search;

pub type EvaluationFn = fn(&Position) -> Evaluation;
pub type SearchFn = fn(&Position, EvaluationFn, u32) -> (Move, Evaluation);

pub trait Bot: DynClone + Send {
    fn search_best_move(&self, position: &Position, search_time: Duration) -> (i32, Move);
}