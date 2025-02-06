use ui::start_ui;

pub mod board;
pub mod player;
mod ui;
pub mod piece;
pub mod player_piece;
pub mod pieces;
pub mod executor;
pub mod evaluation;

fn main() {
    start_ui();
}
