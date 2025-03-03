use std::time::Duration;

use evaluation::Evaluation;

use crate::board::{moove::Move, position::Position};

pub mod evaluation;
pub mod evaluation_funcs;
pub mod search_funcs;
pub mod positioning;
pub mod utils;
pub mod transposition_table;

pub type EvaluationFn = fn(&Position) -> Evaluation;
pub type SearchFn = fn(&Position, EvaluationFn, u32) -> (Move, Evaluation);

pub trait Bot {
    fn search_func(&self) -> fn(position: &Position, search_time: Duration) -> (i32, Move);
}