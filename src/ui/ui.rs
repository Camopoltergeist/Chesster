
use std::{sync::{Arc, RwLock}, time::{Duration, Instant}};

use raylib::{color::Color, ffi::{KeyboardKey, MouseButton}, prelude::{RaylibDraw, RaylibDrawHandle}, RaylibHandle, RaylibThread};

use crate::{board::{game_state::GameState, moove::{Move, PromotingMove}, position::{self, Position}, tile_position::TilePosition}, bot::{evaluation_funcs::{evaluate_material_and_positioning, evaluate_material_and_positioning_debug}, search_funcs::{alpha_beta_search, alpha_beta_search_multithreaded, iterative_deepening}, transposition_table::TranspositionTable}, r#match::Match, piece::PieceType, player::Player, player_piece::PlayerPiece};

use super::{board_renderer::BoardRenderer, text_area::TextArea, texture::{load_circle_texture, load_piece_textures}};

pub struct UI {
	board_renderer: BoardRenderer,
	text_area: TextArea,
	game_match: Match,

	is_debug: bool,

	hovered_tile: Option<TilePosition>,
	selected_tile: Option<TilePosition>,

	background_color: Color,

	promotion_menu_open: bool,
	promoting_move: Option<PromotingMove>,

	transposition_table: Arc<RwLock<TranspositionTable>>,
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
			game_match: Match::new(None, None, Duration::from_secs(2)),
			is_debug: false,
			hovered_tile: None,
			selected_tile: None,
			background_color: Color { r: 0, g: 65, b: 119, a: 255 },
			promotion_menu_open: false,
			promoting_move: None,
			transposition_table: Arc::new(RwLock::new(TranspositionTable::new(10000000)))
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

		let player_str = self.game_match.position().current_player().as_str();

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

		match self.game_match.position().get_game_state() {
			GameState::Ongoing => self.text_area.skip_line(),
			GameState::Checkmate(winner) => self.text_area.draw_line(draw_handle, &format!("{} wins!", winner.as_str())),
			GameState::Stalemate => self.text_area.draw_line(draw_handle, "Draw: Stalemate!"),
		}

		if self.promotion_menu_open {
			self.text_area.draw_line(draw_handle, "Promotion:");
			self.text_area.draw_line(draw_handle, "1: Queen");
			self.text_area.draw_line(draw_handle, "2: Rook");
			self.text_area.draw_line(draw_handle, "3: Knight");
			self.text_area.draw_line(draw_handle, "4: Bishop");
		}

		self.text_area.reset();
	}

	pub fn handle_input(&mut self, rl: &RaylibHandle) {
		if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
			self.toggle_board_perspective();
		}

		self.handle_promotion_menu(rl);

		self.handle_mouse_input(rl);
	}

	fn handle_promotion_menu(&mut self, rl: &RaylibHandle) {
		if !self.promotion_menu_open {
			return;
		}

		let promoting_move = self.promoting_move.clone().unwrap();
		let p = promoting_move.promotion_piece();

		if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
			self.play_move(Move::Promoting(PromotingMove::new(promoting_move.from_position(), promoting_move.to_position(), PlayerPiece::new(p.player(), PieceType::Queen))));
			self.promotion_menu_open = false;
			self.promoting_move = None;
			return;
		}

		if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
			self.play_move(Move::Promoting(PromotingMove::new(promoting_move.from_position(), promoting_move.to_position(), PlayerPiece::new(p.player(), PieceType::Rook))));
			self.promotion_menu_open = false;
			self.promoting_move = None;
			return;
		}

		if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
			self.play_move(Move::Promoting(PromotingMove::new(promoting_move.from_position(), promoting_move.to_position(), PlayerPiece::new(p.player(), PieceType::Knight))));
			self.promotion_menu_open = false;
			self.promoting_move = None;
			return;
		}

		if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
			self.play_move(Move::Promoting(PromotingMove::new(promoting_move.from_position(), promoting_move.to_position(), PlayerPiece::new(p.player(), PieceType::Bishop))));
			self.promotion_menu_open = false;
			self.promoting_move = None;
			return;
		}
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
		if self.promotion_menu_open {
			return;
		}

		if self.selected_tile.is_none() {
			if let Some(piece) = self.game_match.position().get_piece(clicked_tile) {
				if piece.player() != self.game_match.position().current_player() {
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

		let legal_moves = self.game_match.position().generate_legal_moves_for_tile_position(selected_tile);

		for m in legal_moves {
			if m.to_position() == clicked_tile {
				if let Move::Promoting(promoting_move) = &m {
					self.promoting_move = Some(promoting_move.clone());
					self.promotion_menu_open = true;
					return;
				}

				self.play_move(m);

				return;
			}
		}

		if let Some(piece) = self.game_match.position().get_piece(clicked_tile) {
			if piece.player() == self.game_match.position().current_player() {
				self.select_tile(Some(clicked_tile));
				return;
			}
		}

		self.select_tile(None);
	}

	fn play_move(&mut self, m: Move) {
		self.game_match.make_move(m);
		self.board_renderer.set_board(&self.game_match.position().board());
		self.select_tile(None);

		// println!("Eval: {}", evaluate_material_and_positioning(&self.position));

		if self.game_match.position().current_player() != Player::Black {
			return;
		}

		if let GameState::Ongoing = self.game_match.position().get_game_state() {
			let start_time_cacheless = Instant::now();
			let (evaluation, moove) = iterative_deepening(&self.game_match.position(), evaluate_material_and_positioning, Duration::from_secs(2), self.transposition_table.clone());
			let end_time_cacheless = Instant::now();

			println!("WWWWWWWWWWW");
			println!("{} | {}", moove.debug_string(), evaluation);

			println!("Search took {} seconds", end_time_cacheless.duration_since(start_time_cacheless).as_secs_f32());

			if let Ok(mut rwlock) = self.transposition_table.write() {
				println!("TP table entries: {}", rwlock.len());
				// println!("TP table lookups: {}", rwlock.lookups());
				// println!("TP table hit %: {}", rwlock.hit_percent() * 100.0);

				let mut next_pos = self.game_match.position().clone();
				next_pos.make_move(moove);

				let tp = rwlock.get(next_pos.hash().value());

				if let Some(tp) = tp {
					println!("Current transposition from table: {}, {}", tp.depth(), tp.evaluation());
				}

				rwlock.reset_stats();
			}

			// let (ab_eval, ab_move) = alpha_beta_search_multithreaded(&self.position, evaluate_material_and_positioning, 7);
			// println!("Alpha beta: {} | {}", ab_move.debug_string(), ab_eval);
		}
	}

	fn select_tile(&mut self, tile_pos: Option<TilePosition>) {
		self.selected_tile = tile_pos;
		self.board_renderer.set_highlighted_tile(tile_pos);
		self.board_renderer.set_legal_moves(Vec::new());

		if tile_pos.is_none() {
			return;
		}

		let tile_pos = tile_pos.unwrap();

		if let Some(_) = self.game_match.position().get_piece(tile_pos) {
			self.board_renderer.set_legal_moves(self.game_match.position().generate_legal_moves_for_tile_position(tile_pos));
		}
	}

	pub fn set_position(&mut self, position: Position) {
		self.game_match.set_position(&position);
		self.board_renderer.set_board(self.game_match.position().board());
	}

	fn toggle_board_perspective(&mut self) {
		self.board_renderer.swap_player();
	}
}