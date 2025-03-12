use std::{collections::{HashMap, HashSet}, env};

use board::{position::Position, tile_position::TilePosition, zobrist_hash::generate_zobrist_numbers};
use convert::{convert, NewBookEntry};
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
pub mod convert;

fn main() {
    generate_zobrist_numbers();

    let old_book = PolyGlotBook::open("./komodo.bin").unwrap();

    let default_pos = shakmaty::Chess::new();
    let mut new_book: HashMap<u64, NewBookEntry> = HashMap::new();
    let mut visited: HashSet<u64> = HashSet::new();

    convert(&default_pos, &old_book, &mut new_book, &mut visited);

    let out_pos = Position::default();

    println!("Entries: {}", new_book.len());

    for m in new_book.get(&out_pos.hash().value()).unwrap().moves.iter() {
        println!("{:?}", m);
        println!("{}", m.debug_string());
    }

    return;

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
