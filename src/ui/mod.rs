pub mod board_renderer;

use board_renderer::BoardRenderer;
use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

pub fn start_ui() {
	let (mut rl, thread) = raylib::init()
		.vsync()
		.size(WINDOW_WIDTH, WINDOW_HEIGHT)
		.resizable()
		.title("Chesster")
		.build();

	let mut br = BoardRenderer::new(0, 0, WINDOW_HEIGHT, 32, crate::player::Player::Black);

	let bitboard = 1 | 1 << 7;

	br.set_bitboard_overlay(bitboard);

	while !rl.window_should_close() {
		// draw(&mut rl, &thread);
		let mut draw_handle = rl.begin_drawing(&thread);
		br.draw_board(&mut draw_handle);
	}
}

fn draw_bitboard(draw_handle: &mut RaylibDrawHandle, bitboard: u64) {
	const OFF_COLOR: Color = Color {r: 0, g: 0, b: 255, a: 127};
	const ON_COLOR: Color = Color {r: 255, g: 0, b: 0, a: 127};

	for bit_offset in 0..64 {
		let bit = (bitboard & 1 << bit_offset) != 0;

		let color = if bit { ON_COLOR } else { OFF_COLOR };

		let x = bit_offset % 8;
		let y = 7 - bit_offset / 8;

		let pos = get_tile_pos(x, y, 720);

		draw_handle.draw_rectangle(pos.0, pos.1, pos.2, pos.2, color);
	}
}

fn get_tile_pos(column: i32, rank: i32, available_size: i32) -> (i32, i32, i32) {
	const BOARD_SIZE: f32 = 8.0;

	let tile_size = available_size as f32 / BOARD_SIZE;

	let x = column as f32 * tile_size;
	let y = rank as f32 * tile_size;

	return (x as i32, y as i32, tile_size as i32);
}
