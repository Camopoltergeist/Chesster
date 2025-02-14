
use std::time::Instant;

use raylib::{color::Color, ffi::{KeyboardKey, MouseButton}, prelude::{RaylibDraw, RaylibDrawHandle}, RaylibHandle, RaylibThread};

use crate::{board::{game_state::GameState, position::Position, tile_position::TilePosition}, bot::{evaluation_funcs::evaluate_material_and_mobility, search_funcs::{negamax_with_move_chain_multithreaded, print_move_chain}}, player::Player};

use super::{board_renderer::BoardRenderer, text_area::TextArea, texture::{load_circle_texture, load_piece_textures}};

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
		let circle_texture = load_circle_texture(rl, thread);
		let mut board_renderer = BoardRenderer::new(0, 0, rl.get_screen_height(), 32, Player::White, piece_textures, circle_texture);

		let position = Position::default();
		board_renderer.set_board(position.board());

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

		match self.position.get_game_state() {
			GameState::Ongoing => self.text_area.skip_line(),
			GameState::Checkmate(winner) => self.text_area.draw_line(draw_handle, &format!("{} wins!", winner.as_str())),
			GameState::Stalemate => self.text_area.draw_line(draw_handle, "Draw: Stalemate!"),
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

		if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
			if let Some(clicked_tile) = tile_pos_opt {
				self.handle_click_play_mode(clicked_tile);
			}
		}
	}

	fn handle_click_play_mode(&mut self, clicked_tile: TilePosition) {
		if self.selected_tile.is_none() {
			if let Some(piece) = self.position.get_piece(clicked_tile) {
				if piece.player() != self.position.current_player() {
					return;
				}

				self.select_tile(Some(clicked_tile));
			}

			return;
		}

		let selected_tile = self.selected_tile.unwrap();

		if selected_tile == clicked_tile {
			self.select_tile(None);
			return;
		}

		let legal_moves = self.position.generate_legal_moves_for_tile_position(selected_tile);

		for m in legal_moves {
			if m.to_position() == clicked_tile {
				self.position.make_move(m);
				
				self.select_tile(None);
				self.board_renderer.set_board(&self.position.board());

				if let GameState::Ongoing = self.position.get_game_state() {
					let start_time_cacheless = Instant::now();
					let (evaluation, move_chain) = negamax_with_move_chain_multithreaded(&self.position, evaluate_material_and_mobility, 4);
					let end_time_cacheless = Instant::now();

					println!("WWWWWWWWWWW");
					print_move_chain(&move_chain, evaluation);

					println!("Search took {} seconds", end_time_cacheless.duration_since(start_time_cacheless).as_secs_f32());
				}

				return;
			}
		}

		if let Some(piece) = self.position.get_piece(clicked_tile) {
			if piece.player() == self.position.current_player() {
				self.select_tile(Some(clicked_tile));
				return;
			}
		}

		self.select_tile(None);
	}

	fn select_tile(&mut self, tile_pos: Option<TilePosition>) {
		self.selected_tile = tile_pos;
		self.board_renderer.set_highlighted_tile(tile_pos);
		self.board_renderer.set_legal_moves(Vec::new());

		if tile_pos.is_none() {
			return;
		}

		let tile_pos = tile_pos.unwrap();

		if let Some(_) = self.position.get_piece(tile_pos) {
			self.board_renderer.set_legal_moves(self.position.generate_legal_moves_for_tile_position(tile_pos));
		}
	}

	pub fn set_position(&mut self, position: Position) {
		self.position = position;
		self.board_renderer.set_board(self.position.board());
	}

	fn toggle_board_perspective(&mut self) {
		self.board_renderer.swap_player();
	}
}