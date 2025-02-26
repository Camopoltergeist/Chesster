use crate::board::{position::Position, tile_position::TilePosition};

pub fn bishop_pair_bonus(position: &Position) -> i32 {
    let player_board = position
        .board()
        .get_player_bitboard(position.current_player());
    let bishop_board = &position.board().bishops;

    if (player_board.value() & bishop_board.value()).count_ones() == 2 {
        return 35;
    } else {
        return 0;
    }
}

pub fn rook_pair_penalty(position: &Position) -> i32 {
    let player_board = *position
        .board()
        .get_player_bitboard(position.current_player());
    let rook_board = position.board().rooks;
    let player_rook_board = (player_board & rook_board).value();

    if player_rook_board.count_ones() == 2 {
        let rook1_position = TilePosition::from_bit_offset(player_rook_board.trailing_zeros());
        let rook2_position = TilePosition::from_bit_offset(63 - player_rook_board.leading_zeros());

        let col_diff = (rook1_position.column() as i32 - rook2_position.column() as i32).abs() + 1;
        let rank_diff = (rook1_position.rank() as i32 - rook2_position.rank() as i32).abs() + 1;

        if col_diff < rank_diff {
            return -20 / col_diff;
        } else {
            return -20 / rank_diff;
        }
    } else {
        return 0;
    }
}

pub fn no_pawn_penalty(position: &Position) -> i32 {
    let player_pawn_board = *position
        .board()
        .get_player_bitboard(position.current_player())
        & position.board().pawns;

    if player_pawn_board == 0 {
        return -50;
    } else {
        return 0;
    }
}
