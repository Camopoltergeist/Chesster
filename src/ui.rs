use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}, RaylibHandle, RaylibThread};

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

pub fn start_ui() {
	let (mut rl, thread) = raylib::init()
		.vsync()
		.size(WINDOW_WIDTH, WINDOW_HEIGHT)
		.resizable()
		.title("Chesster")
		.build();

	while !rl.window_should_close() {
		draw(&mut rl, &thread);
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

fn draw(rl: &mut RaylibHandle, thread: &RaylibThread) {
	let mut draw_handle = rl.begin_drawing(&thread);

	draw_handle.clear_background(Color::GRAY);
	draw_board(&mut draw_handle, false);
	draw_bitboard(&mut draw_handle, 3);
}

fn get_tile_pos(column: i32, rank: i32, available_size: i32) -> (i32, i32, i32) {
	const BOARD_SIZE: f32 = 8.0;

	let tile_size = available_size as f32 / BOARD_SIZE;

	let x = column as f32 * tile_size;
	let y = rank as f32 * tile_size;

	return (x as i32, y as i32, tile_size as i32);
}

fn draw_board(draw_handle: &mut RaylibDrawHandle, black_view: bool) {
	const UNSCALED_MARGIN: f32 = 32.0;
	const UNSCALED_FONT_SIZE: f32 = 16.0;

	const BOARD_SIZE: i32 = 8;
	const COLUMNS: &str = "ABCDEFGH";

	let screen_height = draw_handle.get_screen_height();
	let scale: f32 = screen_height as f32 / WINDOW_HEIGHT as f32;
	let available_size = (WINDOW_HEIGHT as f32 - UNSCALED_MARGIN * 2.0) * scale;

	let margin = (UNSCALED_MARGIN * scale) as i32;
	let font_size = (UNSCALED_FONT_SIZE * scale) as i32;

	for i in 0..BOARD_SIZE {
		for j in 0..BOARD_SIZE {
			let color = if (i + j) % 2 == 0 { Color::WHITE } else { Color::BLACK };

			let tile_pos = get_tile_pos(i, j, available_size as i32);

			draw_handle.draw_rectangle(tile_pos.0 + margin, tile_pos.1 + margin, tile_pos.2, tile_pos.2, color);
		}
	}

	for i in 0..BOARD_SIZE {
		let rank = if black_view { i + 1 } else { 8 - i };

		let pos = get_tile_pos(0, i, available_size as i32);

		let rank_str = rank.to_string();
		let text_width = draw_handle.measure_text(&rank_str, font_size);

		let x = margin / 2 - text_width / 2;
		let y = margin + pos.1 + pos.2 / 2 - font_size / 2;

		draw_handle.draw_text(&(rank).to_string(), x, y, font_size, Color::BLACK);
	}

	for i in 0..BOARD_SIZE {
		let char_index = if black_view { 7 - i } else { i };
		let column = COLUMNS.chars().nth(char_index as usize).expect("failed to get column letter");

		let pos = get_tile_pos(i, 0, available_size as i32);

		let text_width = draw_handle.measure_text(&column.to_string(), font_size);

		let x = margin + pos.0 + pos.2 / 2 - text_width / 2;
		let y = screen_height - margin / 2 - font_size / 2;

		draw_handle.draw_text(&column.to_string(), x, y, font_size, Color::BLACK);
	}
}