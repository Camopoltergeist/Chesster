use tests::integrity_tests::{integrity_test_depth_3, integrity_test_depth_4};
// use tests::integrity_tests::integrity_test_depth_3;
use ui::start_ui;

pub mod board;
pub mod player;
mod ui;
pub mod piece;
pub mod player_piece;
pub mod pieces;
pub mod bot;

// #[cfg(test)]
pub mod tests;

fn main() {
    integrity_test_depth_4();

    return;
    start_ui();
}
