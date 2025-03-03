use std::env;

use board::zobrist_hash::generate_zobrist_numbers;
use performance_test::performance_test;
use ui::start_ui;

pub mod board;
pub mod player;
mod ui;
pub mod piece;
pub mod player_piece;
pub mod pieces;
pub mod bot;
pub mod perft;

#[cfg(test)]
pub mod tests;
mod performance_test;
pub mod r#match;

fn main() {
    generate_zobrist_numbers();

    let args: Vec<String> = env::args().collect();

    if args.contains(&"--performance-test".to_owned()) {
        let duration = performance_test();

        println!("Performance test took {} seconds.", duration.as_secs_f32());
        return;
    }

    start_ui();
}
