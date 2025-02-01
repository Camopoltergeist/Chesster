
use raylib::{color::Color, ffi::{KeyboardKey, MouseButton}, prelude::{RaylibDraw, RaylibDrawHandle}, RaylibHandle, RaylibThread};

use crate::{board::{board::Board, move_collision::get_collision_mask, position::Position, tile_position::TilePosition}, player::Player};

use super::{board_renderer::BoardRenderer, texture::load_piece_textures};

pub struct UI {
	board_renderer: BoardRenderer,
	position: Option<Position>,

	debug_position: Option<Position>,
	is_debug: bool,

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
			debug_position: None,
			is_debug: false,
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

	pub fn handle_input(&mut self, rl: &RaylibHandle) {
		if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
			self.toggle_debug_position();
		}

		if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
			self.toggle_board_perspective();
		}

		self.handle_mouse_input(rl);
	}
	
	fn handle_mouse_input(&mut self, rl: &RaylibHandle) {
		let mouse_pos = rl.get_mouse_position();

		let tile_pos_opt = self.board_renderer.get_tile_from_pixel_pos(mouse_pos);

		if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
			self.select_tile(tile_pos_opt);
		}
	}

	fn select_tile(&mut self, tile_pos: Option<TilePosition>) {
		self.selected_tile = tile_pos;
		self.board_renderer.set_highlighted_tile(tile_pos);
		self.board_renderer.set_bitboard_overlay(None);

		let shown_position = self.shown_position();

		if shown_position.is_none() {
			return;
		}

		let position = shown_position.unwrap();

		if tile_pos.is_none() {
			return;
		}

		let tile_pos = tile_pos.unwrap();

		if let Some(_) = position.get_piece(tile_pos) {
			let mask = get_collision_mask(position.board().clone(), tile_pos);
			self.board_renderer.set_bitboard_overlay(Some(mask));
		}
	}

	pub fn shown_position(&self) -> Option<&Position> {
		if self.is_debug { self.debug_position.as_ref() } else { self.position.as_ref() } 
	}

	pub fn position(&self) -> Option<&Position> {
		self.position.as_ref()
	}

	pub fn set_position(&mut self, position: Option<Position>) {
		self.position = position;
	}

	fn set_rendered_position(&mut self, position: Option<Position>) {
		if let Some(position) = position {
			self.board_renderer.set_board(Some(position.board()));
		}
		else {
			self.board_renderer.set_board(None);
		}
	}

	fn toggle_board_perspective(&mut self) {
		self.board_renderer.swap_player();
	}

	fn toggle_debug_position(&mut self) {
		let not_current_position = if self.is_debug {
			self.position.clone()
		}
		else {
			self.debug_position.clone()
		};

		self.set_rendered_position(not_current_position);
		self.is_debug = !self.is_debug;
	}
}