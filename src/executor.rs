use crate::board::{board::Board, moove::Move};

pub type EvaluationFn = fn(&Board) -> f32;
pub type SearchFn = fn(&Board) -> Move;

