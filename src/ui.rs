use raylib::{color::Color, prelude::RaylibDraw};

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

pub fn start_ui() {
	let (mut rl, thread) = raylib::init()
		.size(WINDOW_WIDTH, WINDOW_HEIGHT)
		.title("Chesster")
		.build();

	while !rl.window_should_close() {
		let mut draw_handle = rl.begin_drawing(&thread);

		draw_handle.clear_background(Color::WHITE);
		draw_handle.draw_text("Hello World!", 12, 12, 32, Color::BLACK);
	}
}