use crate::{
    board::{bitboard::Bitboard, position::Position, tile_position::TilePosition},
    player::Player,
};

pub fn calculate_game_phase(position: &Position) -> (i32, i32) {
    let mut position_materials = 0;
    let full_board = 24;

    position_materials += position.board().queens.value().count_ones() * 4;
    position_materials += position.board().rooks.value().count_ones() * 2;
    position_materials += position.board().bishops.value().count_ones();
    position_materials += position.board().knights.value().count_ones();

    let midgame_percentage = (position_materials * 100 / full_board) as i32;
    let endgame_percentage = 100 - midgame_percentage;

    (midgame_percentage, endgame_percentage)
}

pub fn bishop_pair_bonus(position: &Position) -> i32 {
    const BISHOP_PAIR_BONUS: i32 = 35;
    let mut score = 0;

    let player_board = position
        .board()
        .get_player_bitboard(position.current_player());

    let enemy_board = position
        .board()
        .get_player_bitboard(position.current_player().opposite());

    let bishop_board = &position.board().bishops;

    if (player_board.value() & bishop_board.value()).count_ones() == 2 {
        score += BISHOP_PAIR_BONUS;
    }
    if (enemy_board.value() & bishop_board.value()).count_ones() == 2 {
        score -= BISHOP_PAIR_BONUS;
    }

    score
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

pub fn passed_pawns_bonus(position: &Position) -> i32 {
    const PASSED_PAWN_BONUS: i32 = 20;
    let mut score = 0;

    let pawn_board = position.board().pawns;

    let white_player_board = *position.board().get_player_bitboard(Player::White);

    let black_player_board = *position.board().get_player_bitboard(Player::Black);

    let mut white_pawn_board = pawn_board & white_player_board;
    let mut black_pawn_board = pawn_board & black_player_board;

    while white_pawn_board != 0 {
        let bit_offset = white_pawn_board.pop_lsb();
        let pawn_position = TilePosition::from_bit_offset(bit_offset);

        let mut passed_check_mask = Bitboard::generate_column_mask(pawn_position.column())
            << (pawn_position.rank() + 1) as u64 * 8;

        if pawn_position.column() != 0 {
            passed_check_mask |= Bitboard::generate_column_mask(pawn_position.column() - 1)
                << (pawn_position.rank() + 1) as u64 * 8
        }

        if pawn_position.column() != 7 {
            passed_check_mask |= Bitboard::generate_column_mask(pawn_position.column() + 1)
                << (pawn_position.rank() + 1) as u64 * 8
        }

        if passed_check_mask & pawn_board == 0 {
            score += PASSED_PAWN_BONUS;
        }
    }

    while black_pawn_board != 0 {
        let bit_offset = black_pawn_board.pop_lsb();
        let pawn_position = TilePosition::from_bit_offset(bit_offset);

        let mut passed_check_mask = Bitboard::generate_column_mask(pawn_position.column())
            >> (8 - pawn_position.rank()) as u64 * 8;

        if pawn_position.column() != 0 {
            passed_check_mask |= Bitboard::generate_column_mask(pawn_position.column() - 1)
                >> (8 - pawn_position.rank()) as u64 * 8
        }

        if pawn_position.column() != 7 {
            passed_check_mask |= Bitboard::generate_column_mask(pawn_position.column() + 1)
                >> (8 - pawn_position.rank()) as u64 * 8
        }

        if passed_check_mask & pawn_board == 0 {
            score -= PASSED_PAWN_BONUS;
        }
    }

    if position.current_player() == Player::White {
        score
    } else {
        -score
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
