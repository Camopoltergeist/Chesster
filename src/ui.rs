use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}, RaylibHandle, RaylibThread};

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

pub fn start_ui() {
	let (mut rl, thread) = raylib::init()
		.vsync()
		.size(WINDOW_WIDTH, WINDOW_HEIGHT)
		.title("Chesster")
		.build();

	while !rl.window_should_close() {
		draw(&mut rl, &thread);
	}
}

fn draw(rl: &mut RaylibHandle, thread: &RaylibThread) {
	let mut draw_handle = rl.begin_drawing(&thread);

	draw_handle.clear_background(Color::GRAY);
	draw_board(&mut draw_handle, false);
}

fn draw_board(draw_handle: &mut RaylibDrawHandle, black_view: bool) {
	const MARGIN: i32 = 32;

	let start_x = MARGIN;
	let start_y = MARGIN;

	let tile_size = (WINDOW_HEIGHT - MARGIN * 2) / 8;

	for i in 0..8 {
		for j in 0..8 {
			let color = if (i + j) % 2 == 0 { Color::WHITE } else { Color::BLACK };

			let x = i * tile_size + start_x;
			let y = j * tile_size + start_y;

			draw_handle.draw_rectangle(x, y, tile_size, tile_size, color);
		}
	}

	const FONT_SIZE: i32 = 16;

	for i in 0..8 {
		let rank = if black_view { i + 1 } else { 8 - i };

		let x = MARGIN / 2 - FONT_SIZE / 4;
		let y = i * tile_size + start_y + tile_size / 2 - FONT_SIZE / 2;

		draw_handle.draw_text(&(rank).to_string(), x, y, FONT_SIZE, Color::BLACK);
	}

	const COLUMNS: &str = "ABCDEFGH";

	for i in 0..8 {
		let char_index = if black_view { 7 - i } else { i };
		let column = COLUMNS.chars().nth(char_index as usize).expect("failed to get column letter");

		let x = i * tile_size + start_x + tile_size / 2 - FONT_SIZE / 4;
		let y = WINDOW_HEIGHT - MARGIN / 2 - FONT_SIZE / 2;

		draw_handle.draw_text(&column.to_string(), x, y, FONT_SIZE, Color::BLACK);
	}
}