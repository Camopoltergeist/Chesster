pub mod board_renderer;
pub mod texture;
pub mod text_area;
pub mod ui;

use std::time::Duration;

use ui::UI;

use crate::{board::position::Position, bot::{evaluation_funcs::evaluate_material_and_positioning, iterative_deepening_search::IterativeDeepeningSearch, Bot}, r#match::Match};

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

pub fn start_ui(white_bot: bool, black_bot: bool) {
	let (mut rl, thread) = raylib::init()
		.vsync()
		.size(WINDOW_WIDTH, WINDOW_HEIGHT)
		.resizable()
		.title("Chesster")
		.build();

	let position = Position::default();

	let white_bot: Option<Box<dyn Bot>> = if white_bot { Some(Box::new(IterativeDeepeningSearch::new(evaluate_material_and_positioning))) } else { None };
	let black_bot: Option<Box<dyn Bot>> = if black_bot { Some(Box::new(IterativeDeepeningSearch::new(evaluate_material_and_positioning))) } else { None };

	let game_match = Match::new(&position, white_bot, black_bot, Duration::from_secs(2));

	let mut ui = UI::new(&mut rl, &thread, game_match);

	while !rl.window_should_close() {
		ui.handle_input(&rl);
		ui.make_bot_move_if_ready();
		ui.draw(&mut rl, &thread);
	}
}
