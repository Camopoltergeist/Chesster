
use raylib::{color::Color, prelude::RaylibDraw, RaylibHandle, RaylibThread};

use crate::{board::{position::Position, tile_position::TilePosition}, player::Player};

use super::{board_renderer::BoardRenderer, texture::load_piece_textures};

pub struct UI {
	rl: RaylibHandle,
	thread: RaylibThread,

	board_renderer: BoardRenderer,
	position: Option<Position>,

	selected_tile: Option<TilePosition>,

	background_color: Color,
}

impl UI {
	pub fn new() -> Self {
		let (mut rl, thread) = raylib::init()
			.vsync()
			.size(1280, 720)
			.resizable()
			.title("Chesster")
			.build();

		let piece_textures = load_piece_textures(&mut rl, &thread);
		let board_renderer = BoardRenderer::new(0, 0, rl.get_screen_height(), 32, Player::White, piece_textures);

		Self {
			rl,
			thread,
			board_renderer,
			position: None,
			selected_tile: None,
			background_color: Color { r: 0, g: 65, b: 119, a: 255 },
		}
	}

	pub fn start_loop(&mut self) {
		while !self.rl.window_should_close() {
			let mut draw_handle = self.rl.begin_drawing(&self.thread);
			draw_handle.clear_background(self.background_color);

			self.board_renderer.draw(&mut draw_handle);
		}
	}
}