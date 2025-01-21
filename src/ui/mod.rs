pub mod board_renderer;

use board_renderer::BoardRenderer;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

pub fn start_ui() {
	let (mut rl, thread) = raylib::init()
		.vsync()
		.size(WINDOW_WIDTH, WINDOW_HEIGHT)
		.resizable()
		.title("Chesster")
		.build();

	let mut br = BoardRenderer::new(0, 0, WINDOW_HEIGHT, 32, crate::player::Player::Black);

	let bitboard = 1 | 1 << 7;

	br.set_bitboard_overlay(bitboard);

	while !rl.window_should_close() {
		let mut draw_handle = rl.begin_drawing(&thread);
		br.draw_board(&mut draw_handle);
	}
}
