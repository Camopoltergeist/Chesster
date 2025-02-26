use crate::board::position::Position;

pub fn bishop_pair_bonus(position: Position) -> i32 {
    let player_board = position.board().get_player_bitboard(position.current_player()).value();
    let bishop_board = position.board().bishops.value();

    if (player_board & bishop_board).count_ones() == 2 {
        return 50
    } else {
        return 0
    }
}