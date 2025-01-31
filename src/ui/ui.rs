
use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}, RaylibHandle, RaylibThread};

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
		let mut board_renderer = BoardRenderer::new(0, 0, rl.get_screen_height(), 32, Player::White, piece_textures);

		let position = Position::default();
		board_renderer.set_board(Some(position.board()));

		Self {
			board_renderer,
			position: Some(position),
			selected_tile: None,
			background_color: Color { r: 0, g: 65, b: 119, a: 255 },
		}
	}

	pub fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
		let mut draw_handle = rl.begin_drawing(thread);

		draw_handle.clear_background(self.background_color);

		self.draw_board(&mut draw_handle);
		
	}

	fn draw_board(&mut self, draw_handle: &mut RaylibDrawHandle) {
		let min_dimension = i32::min(draw_handle.get_screen_width(), draw_handle.get_screen_height());
		self.board_renderer.set_size(min_dimension);

		self.board_renderer.draw(draw_handle);
	}

	pub fn position(&self) -> Option<&Position> {
		self.position.as_ref()
	}

	pub fn set_position(&mut self, position: Option<Position>) {
		self.position = position;
	}
}