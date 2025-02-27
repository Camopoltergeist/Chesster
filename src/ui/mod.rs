pub mod board_renderer;
pub mod texture;
pub mod text_area;
pub mod ui;

use ui::UI;

use crate::board::position::{self, Position};

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

pub fn start_ui() {
	let (mut rl, thread) = raylib::init()
		.vsync()
		.size(WINDOW_WIDTH, WINDOW_HEIGHT)
		.resizable()
		.title("Chesster")
		.build();

	let mut ui = UI::new(&mut rl, &thread);

	let position = Position::from_fen_str("8/8/8/3k4/8/R7/1R6/K7 b - - 0 1").unwrap();

	ui.set_position(position);

	while !rl.window_should_close() {
		ui.handle_input(&rl);
		ui.draw(&mut rl, &thread);
	}
}
