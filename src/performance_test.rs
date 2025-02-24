use std::{hint::black_box, time::{Duration, Instant}};

use crate::{board::{moove::Move, position::Position}, bot::{evaluation_funcs::evaluate_material_and_positioning, search_funcs::alpha_beta_search}};

pub fn performance_test() -> Duration {
    let mut position = Position::default();

    let start_time = Instant::now();

    position.make_move(Move::debug_new_basic("e2", "e4"));
    black_box(alpha_beta_search(&position, evaluate_material_and_positioning, 6));

    position.make_move(Move::debug_new_basic("e7", "e5"));
    black_box(alpha_beta_search(&position, evaluate_material_and_positioning, 6));

    let end_time = Instant::now();

    return end_time - start_time;
}