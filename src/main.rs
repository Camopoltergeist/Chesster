use std::{collections::{HashMap, HashSet}, env};

use board::{position::Position, tile_position::TilePosition, zobrist_hash::generate_zobrist_numbers};
use opening_book::{convert, load_opening_book};
use jja::{polyglot::to_move, polyglotbook::PolyGlotBook};
use performance_test::performance_test;
use shakmaty::zobrist::{Zobrist64, ZobristHash};
use ui::start_ui;

pub mod board;
pub mod player;
mod ui;
pub mod piece;
pub mod player_piece;
pub mod pieces;
pub mod bot;
pub mod perft;

#[cfg(test)]
pub mod tests;
mod performance_test;
pub mod r#match;
pub mod opening_book;

fn main() {
    generate_zobrist_numbers();

    let args: Vec<String> = env::args().collect();

    if args.contains(&"--performance-test".to_owned()) {
        let duration = performance_test();

        println!("Performance test took {} seconds.", duration.as_secs_f32());
        return;
    }

    let white_bot = args.contains(&"--white_bot".to_owned());
    let black_bot = args.contains(&"--black_bot".to_owned());

    start_ui(white_bot, black_bot);
}
