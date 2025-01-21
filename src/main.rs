use board::Board;

pub mod board;

fn main() {
    let board = Board::default();


    print_bitboard(board.pawns.0);
}

fn print_bitboard(bitboard: u64) {
    let bin_str: String = format!("{:064b}", bitboard).chars().rev().collect();
    let out_str = format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        &bin_str[56..64],
        &bin_str[48..56],
        &bin_str[40..48],
        &bin_str[32..40],
        &bin_str[24..32],
        &bin_str[16..24],
        &bin_str[8..16],
        &bin_str[0..8],
    );

    println!("{}", out_str);
}
