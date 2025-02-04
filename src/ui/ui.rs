
use raylib::{color::Color, ffi::{KeyboardKey, MouseButton}, prelude::{RaylibDraw, RaylibDrawHandle}, RaylibHandle, RaylibThread};

use crate::{board::{bitboard::Bitboard, moove::Move, move_collision::get_collision_mask, position::Position, tile_position::TilePosition}, player::Player};

use super::{board_renderer::BoardRenderer, text_area::TextArea, texture::load_piece_textures};

pub struct UI {
	board_renderer: BoardRenderer,
	text_area: TextArea,
	position: Position,

	is_debug: bool,

	hovered_tile: Option<TilePosition>,
	selected_tile: Option<TilePosition>,

	background_color: Color,
}

impl UI {
	pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
		let piece_textures = load_piece_textures(rl, thread);
		let mut board_renderer = BoardRenderer::new(0, 0, rl.get_screen_height(), 32, Player::White, piece_textures);

		let position = Position::default();
		board_renderer.set_board(Some(position.board()));

		position.print_all_legal_moves();

		rl.gui_load_style_default();

		Self {
			text_area: TextArea::new(board_renderer.size(), board_renderer.margin(), 20),
			board_renderer,
			position,
			is_debug: false,
			hovered_tile: None,
			selected_tile: None,
			background_color: Color { r: 0, g: 65, b: 119, a: 255 },
		}
	}

	pub fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
		let mut draw_handle = rl.begin_drawing(thread);

		draw_handle.clear_background(self.background_color);

		self.draw_board(&mut draw_handle);
		self.draw_text(&mut draw_handle);
	}

	fn draw_board(&mut self, draw_handle: &mut RaylibDrawHandle) {
		let min_dimension = i32::min(draw_handle.get_screen_width(), draw_handle.get_screen_height());
		self.board_renderer.set_size(min_dimension);

		self.board_renderer.draw(draw_handle);
	}

	fn draw_text(&mut self, draw_handle: &mut RaylibDrawHandle) {
		if self.is_debug {
			self.text_area.draw_line(draw_handle, "Debug Board");
		}

		let player_str = self.position.current_player().as_str();

		self.text_area.draw_line(draw_handle, format!("Current player: {}", &player_str).as_str());


		if let Some(hovered_tile) = self.hovered_tile {
			self.text_area.draw_line(draw_handle, &hovered_tile.notation_string());
		}
		else {
			self.text_area.skip_line();
		}

		if let Some(selected_tile) = self.selected_tile {
			self.text_area.draw_line(draw_handle, format!("Selected tile: {}", &selected_tile.notation_string()).as_str());
		}
		else {
			self.text_area.skip_line();
		}

		self.text_area.reset();
	}

	pub fn handle_input(&mut self, rl: &RaylibHandle) {
		if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
			self.toggle_board_perspective();
		}

		self.handle_mouse_input(rl);
	}
	
	fn handle_mouse_input(&mut self, rl: &RaylibHandle) {
		let mouse_pos = rl.get_mouse_position();

		let tile_pos_opt = self.board_renderer.get_tile_from_pixel_pos(mouse_pos);
		self.hovered_tile = tile_pos_opt;

		// TODO: Holy fuck, this is a mess
		if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
			if let Some(selected_tile) = self.selected_tile {
				if let Some(clicked_tile) = tile_pos_opt {
					let moove = Move::new(selected_tile, clicked_tile);

					let move_result = self.position.make_move(moove);

					if move_result.is_ok() {
						self.board_renderer.set_board(Some(self.position.board()));
						self.select_tile(None);
						self.position.print_all_legal_moves();
						return;
					}
				}
			}

			self.select_tile(tile_pos_opt);
		}
	}

	fn select_tile(&mut self, tile_pos: Option<TilePosition>) {
		self.selected_tile = tile_pos;
		self.board_renderer.set_highlighted_tile(tile_pos);
		self.board_renderer.set_bitboard_overlay(None);

		if tile_pos.is_none() {
			return;
		}

		let tile_pos = tile_pos.unwrap();

		if let Some(_) = self.position.get_piece(tile_pos) {
			let mask = get_collision_mask(self.position.board().clone(), tile_pos);
			self.board_renderer.set_bitboard_overlay(Some(mask));
		}
	}

	pub fn position(&self) -> &Position {
		&self.position
	}

	pub fn set_position(&mut self, position: Position) {
		self.position = position;
		self.board_renderer.set_board(Some(self.position.board()));
	}

	fn toggle_board_perspective(&mut self) {
		self.board_renderer.swap_player();
	}
}