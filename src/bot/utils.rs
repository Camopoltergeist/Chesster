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

pub fn bishop_pair_bonus(position: &Position, game_phase: (i32, i32)) -> i32 {
    const BISHOP_PAIR_BONUS: (i32, i32) = ( 22, 88);
    let mut score = 0;

    let player_board = position
        .board()
        .get_player_bitboard(position.current_player());

    let enemy_board = position
        .board()
        .get_player_bitboard(position.current_player().opposite());

    let bishop_board = &position.board().bishops;

    if (player_board.value() & bishop_board.value()).count_ones() == 2 {
        score += (BISHOP_PAIR_BONUS.0 * game_phase.0 + BISHOP_PAIR_BONUS.1 * game_phase.1) / 100;
    }
    if (enemy_board.value() & bishop_board.value()).count_ones() == 2 {
        score -= (BISHOP_PAIR_BONUS.0 * game_phase.0 + BISHOP_PAIR_BONUS.1 * game_phase.1) / 100;
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

pub fn rook_open_column_bonus(position: &Position, game_phase: (i32, i32)) -> i32 {
    const OPEN_COLUMN_BONUS: (i32, i32) = (8, 20);

    let pawn_board = position.board().pawns;
    let mut score = 0;

    let white_board = *position.board().get_player_bitboard(Player::White);
    let mut white_rook_board = white_board & position.board().rooks;

    let black_board = *position.board().get_player_bitboard(Player::Black);
    let mut black_rook_board = black_board & position.board().rooks;

    while white_rook_board != 0 {
        let bit_offset = white_rook_board.pop_lsb();
        let rook_position = TilePosition::from_bit_offset(bit_offset);
        let column_check_mask = Bitboard::generate_column_mask(rook_position.column());

        if column_check_mask & pawn_board == 0 {
            score += OPEN_COLUMN_BONUS.0 * game_phase.0 + OPEN_COLUMN_BONUS.1 * game_phase.1;
        } else if column_check_mask & (pawn_board & white_board) == 0 {
            score += (OPEN_COLUMN_BONUS.0 * game_phase.0 + OPEN_COLUMN_BONUS.1 * game_phase.1) / 2
        }
    }

    while black_rook_board != 0 {
        let bit_offset = black_rook_board.pop_lsb();
        let rook_position = TilePosition::from_bit_offset(bit_offset);
        let column_check_mask = Bitboard::generate_column_mask(rook_position.column());
        
        if column_check_mask & pawn_board == 0 {
            score -= OPEN_COLUMN_BONUS.0 * game_phase.0 + OPEN_COLUMN_BONUS.1 * game_phase.1;
        } else if column_check_mask & (pawn_board & black_board) == 0 {
            score -= (OPEN_COLUMN_BONUS.0 * game_phase.0 + OPEN_COLUMN_BONUS.1 * game_phase.1) / 2
        }
    }

    if position.current_player() == Player::White {
        score / 100
    } else {
        -score / 100
    }
}

pub fn passed_pawns_bonus(position: &Position) -> i32 {
    const PASSED_PAWN_BONUS: i32 = 20;
    let mut score = 0;

    let pawn_board = position.board().pawns;

    let white_player_board = *position.board().get_player_bitboard(Player::White);

    let black_player_board = *position.board().get_player_bitboard(Player::Black);

    let white_pawn_board = pawn_board & white_player_board;
    let mut white_pawn_board_copy = white_pawn_board;
    let black_pawn_board = pawn_board & black_player_board;
    let mut black_pawn_board_copy = black_pawn_board;

    while white_pawn_board_copy != 0 {
        let bit_offset = white_pawn_board_copy.pop_lsb();
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

        if passed_check_mask & black_pawn_board == 0 {
            score += PASSED_PAWN_BONUS;
        }
    }

    while black_pawn_board_copy != 0 {
        let bit_offset = black_pawn_board_copy.pop_lsb();
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

        if passed_check_mask & white_pawn_board == 0 {
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
    const NO_PAWN_PENALTY: i32 = -50;
    let mut score = 0;

    let white_pawn_board =
        *position.board().get_player_bitboard(Player::White) & position.board().pawns;

    let black_pawn_board =
        *position.board().get_player_bitboard(Player::Black) & position.board().pawns;

    if white_pawn_board == 0 {
        score += NO_PAWN_PENALTY;
    }
    if black_pawn_board == 0 {
        score -= NO_PAWN_PENALTY
    }

    if position.current_player() == Player::White {
        score
    } else {
        -score
    }
}
