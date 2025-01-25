pub mod board_renderer;
pub mod texture;

use board_renderer::BoardRenderer;
use raylib::{color::Color, ffi::KeyboardKey, prelude::RaylibDraw};
use texture::load_piece_textures;

use crate::{board::board::Board, piece::Piece, player::Player};

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

	let mut br = BoardRenderer::new(0, 0, WINDOW_HEIGHT, 32, Player::White, piece_textures);

	let mut board = Board::default();
	board.set_piece_to_offset(Player::White, Piece::Queen, 0);

	br.set_board(Some(&board));

	let mut just_pressed = false;

	while !rl.window_should_close() {
		if rl.is_key_down(KeyboardKey::KEY_SPACE) {
			if !just_pressed {
				br.swap_player();
			}

			just_pressed = true;
		}
		else {
			just_pressed = false;
		}

		let mut draw_handle = rl.begin_drawing(&thread);
		draw_handle.clear_background(Color { r: 0, g: 65, b: 119, a: 255 });
		br.draw(&mut draw_handle);
	}
}
