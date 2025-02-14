use board::position::Position;
use perft::perft;
use ui::start_ui;

pub mod board;
pub mod player;
mod ui;
pub mod piece;
pub mod player_piece;
pub mod pieces;
pub mod bot;

#[cfg(test)]
pub mod tests;
pub mod perft;

fn main() {
    // let position = Position::from_fen_str("rnbq1k1r/pp1Pbppp/8/2p5/2B5/4B3/PPPQN1PP/RN2K2n w Q - 0 10").unwrap();
    // let perft_vec = perft(&position, 1);

    // for m in &perft_vec {
    //     println!("{} | {}", m.0.debug_string(), m.1);
    // }

    // let positions_searched = perft_vec
    //     .iter()
    //     .map(|e| e.1)
    //     .reduce(|acc, e| acc + e)
    //     .unwrap_or(0);

    // println!("Searched {}", positions_searched);

    // return;

    start_ui();
}
