pub mod board_renderer;
pub mod texture;
pub mod text_area;
pub mod ui;

use board_renderer::BoardRenderer;
use raylib::{color::Color, ffi::{KeyboardKey, MouseButton}, prelude::RaylibDraw};
use text_area::TextArea;
use texture::load_piece_textures;
use ui::UI;

use crate::{board::{board::Board, move_collision::get_collision_mask, move_mask::get_move_mask, tile_position::TilePosition}, debug_board::create_debug_board, piece::Piece, player::Player, player_piece::PlayerPiece};

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

pub fn start_ui() {
	let mut ui = UI::new();
	ui.start_loop();

	return;

	let (mut rl, thread) = raylib::init()
		.vsync()
		.size(WINDOW_WIDTH, WINDOW_HEIGHT)
		.resizable()
		.title("Chesster")
		.build();

	let piece_textures = load_piece_textures(&mut rl, &thread);

	let mut br = BoardRenderer::new(0, 0, WINDOW_HEIGHT, 32, Player::White, piece_textures);

	let board = Board::default();
	let debug_board = create_debug_board();

	let mut is_debug = false;

	br.set_board(Some(&board));

	let mut text_area = TextArea::new(br.size(), br.margin(), 20);

	let mut selected_tile: Option<TilePosition> = None;

	while !rl.window_should_close() {
		if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
			br.swap_player();
		}

		if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
			if is_debug { br.set_board(Some(&board)); } else { br.set_board(Some(&debug_board)); };
			br.set_bitboard_overlay(None);
			br.set_highlighted_tile(None);
			selected_tile = None;
			is_debug = !is_debug;
		}

		let mouse_pos = rl.get_mouse_position();
		let tile_pos_opt = br.get_tile_from_pixel_pos(mouse_pos);

		if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
			if let Some(board) = br.board() {
				if let Some(tile_pos) = tile_pos_opt {
					if let Some(piece) = board.get_piece(tile_pos) {
						let mask = get_collision_mask(board.clone(), tile_pos);
						
						br.set_bitboard_overlay(Some(mask));
					}
					else {
						br.set_bitboard_overlay(None);
					}
				}
				else {
					br.set_bitboard_overlay(None);
				}
			}

			br.set_highlighted_tile(tile_pos_opt);
			selected_tile = tile_pos_opt;
		}

		let min_dimension = i32::min(rl.get_screen_width(), rl.get_screen_height());
		br.set_size(min_dimension);

		let mut draw_handle = rl.begin_drawing(&thread);

		if let Some(tile_pos) = tile_pos_opt {
			text_area.draw_line(&mut draw_handle, &format!("{}", tile_pos.notation_string()));
		}

		if let Some(tile_pos) = selected_tile {
			text_area.draw_line(&mut draw_handle, &format!("Selected tile: {}", tile_pos.notation_string()));
		}

		if is_debug {
			text_area.draw_line(&mut draw_handle, "Debug board");
		}

		draw_handle.clear_background(Color { r: 0, g: 65, b: 119, a: 255 });
		br.draw(&mut draw_handle);

		text_area.reset();
	}
}
