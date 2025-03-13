use std::time::{Duration, Instant};

use crate::{board::{moove::Move, position::Position}, bot::{evaluation_funcs::evaluate_material_and_positioning, iterative_deepening_search::IterativeDeepeningSearch}, r#match::Match};

pub fn performance_test() -> Duration {
    let mut match_ = Match::new(&Position::default(), Some(Box::new(IterativeDeepeningSearch::new(evaluate_material_and_positioning, true))), Some(Box::new(IterativeDeepeningSearch::new(evaluate_material_and_positioning, true))), Duration::from_secs(10000));

    let start_time = Instant::now();

    match_.make_move(Move::debug_new_basic("e2", "e4"));
    match_.wait_until_calculation_finished();

    match_.make_move(Move::debug_new_basic("e7", "e5"));
    match_.wait_until_calculation_finished();

    let end_time = Instant::now();

    return end_time - start_time;
}