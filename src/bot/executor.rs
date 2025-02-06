use crate::{board::{moove::Move, position::Position}, bot::evaluation::Evaluation};

pub type EvaluationFn = fn(&Position) -> Evaluation;
pub type SearchFn = fn(&Position, EvaluationFn, u32) -> (Move, Evaluation);
