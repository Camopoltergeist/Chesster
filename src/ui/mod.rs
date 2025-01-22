pub mod board_renderer;
pub mod texture;

use board_renderer::BoardRenderer;
use texture::load_piece_textures;

use crate::board::Board;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

pub fn start_ui() {
	let (mut rl, thread) = raylib::init()
		.vsync()
		.size(WINDOW_WIDTH, WINDOW_HEIGHT)
		.resizable()
		.title("Chesster")
		.build();

	let piece_textures = load_piece_textures(&mut rl, &thread);

	println!("{:?}", piece_textures);

	let mut br = BoardRenderer::new(0, 0, WINDOW_HEIGHT, 32, crate::player::Player::Black, piece_textures);

	let board = Board::default();

	br.set_bitboard_overlay(board.kings.0);

	while !rl.window_should_close() {
		let mut draw_handle = rl.begin_drawing(&thread);
		br.draw_board(&mut draw_handle);
	}
}
