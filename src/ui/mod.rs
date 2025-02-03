pub mod board_renderer;
pub mod texture;
pub mod text_area;
pub mod ui;

use ui::UI;

use crate::board::position::Position;

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

	let position = Position::default();

	ui.set_position(position);

	while !rl.window_should_close() {
		ui.handle_input(&rl);
		ui.draw(&mut rl, &thread);
	}
}
