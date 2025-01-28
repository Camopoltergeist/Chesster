pub mod board_renderer;
pub mod texture;

use board_renderer::BoardRenderer;
use raylib::{color::Color, ffi::{KeyboardKey, MouseButton}, prelude::RaylibDraw};
use texture::load_piece_textures;

use crate::{board::board::Board, player::Player};

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

	let board = Board::default();

	br.set_board(Some(&board));

	while !rl.window_should_close() {
		if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
			br.swap_player();
		}

		if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
			let mouse_pos = rl.get_mouse_position();
			let tile = br.get_tile_from_pixel_pos(mouse_pos);

			if let Some(board) = br.board() {
				if let Some(tile_pos) = tile {
					let piece = board.get_piece(tile_pos.0, tile_pos.1);
					println!("{:?}", piece);
				}
			}

			br.set_highlighted_tile(tile);
		}

		let min_dimension = i32::min(rl.get_screen_width(), rl.get_screen_height());
		br.set_size(min_dimension);

		let mut draw_handle = rl.begin_drawing(&thread);
		draw_handle.clear_background(Color { r: 0, g: 65, b: 119, a: 255 });
		br.draw(&mut draw_handle);
	}
}
