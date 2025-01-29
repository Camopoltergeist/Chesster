use ui::start_ui;

pub mod board;
pub mod player;
mod ui;
pub mod piece;
pub mod player_piece;
pub mod debug_board;

fn main() {
    start_ui();
}
