
use raylib::{color::Color, prelude::RaylibDrawHandle, RaylibHandle, RaylibThread};

use crate::{board::{position::Position, tile_position::TilePosition}, player::Player};

use super::{board_renderer::BoardRenderer, texture::load_piece_textures};

pub struct UI {
	board_renderer: BoardRenderer,
	position: Option<Position>,

	selected_tile: Option<TilePosition>,

	background_color: Color,
}

impl UI {
	pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
		let piece_textures = load_piece_textures(rl, thread);
		let board_renderer = BoardRenderer::new(0, 0, rl.get_screen_height(), 32, Player::White, piece_textures);

		Self {
			board_renderer,
			position: None,
			selected_tile: None,
			background_color: Color { r: 0, g: 65, b: 119, a: 255 },
		}
	}

	fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
		let mut draw_handle = rl.begin_drawing(thread);

		self.draw_board(&mut draw_handle);
		
	}

	fn draw_board(&mut self, draw_handle: &mut RaylibDrawHandle) {
		let min_dimension = i32::min(draw_handle.get_screen_width(), draw_handle.get_screen_height());
		self.board_renderer.set_size(min_dimension);

		self.board_renderer.draw(draw_handle);
	}
}