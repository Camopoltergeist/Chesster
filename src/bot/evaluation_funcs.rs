use crate::board::{game_state::GameState, position::Position};

use super::{evaluation::Evaluation, utils::{bishop_pair_bonus, calculate_game_phase, rook_open_column_bonus}};

pub fn evaluate_material_only(position: &Position) -> Evaluation {
	let own_material = position.board().get_material_for_player(position.current_player()) as f32;
	let opponent_material = position.board().get_material_for_player(position.current_player().opposite()) as f32;

	Evaluation::Score(own_material - opponent_material)
}

pub fn evaluate_material_and_checkmates(position: &Position) -> Evaluation {
	match position.get_game_state() {
		GameState::Checkmate(winner) => if position.current_player() == winner { Evaluation::Checkmate(true) } else { Evaluation::Checkmate(false) },
		GameState::Stalemate => Evaluation::Stalemate,
		GameState::Ongoing => {
			let own_material = position.board().get_material_for_player(position.current_player()) as f32;
			let opponent_material = position.board().get_material_for_player(position.current_player().opposite()) as f32;

			Evaluation::Score(own_material - opponent_material)
		}
	}
}

pub fn evaluate_material_and_mobility(position: &Position) -> Evaluation {
	match position.get_game_state() {
		GameState::Checkmate(winner) => if position.current_player() == winner { Evaluation::Checkmate(true) } else { Evaluation::Checkmate(false) },
		GameState::Stalemate => Evaluation::Stalemate,
		GameState::Ongoing => {
			let own_material = position.board().get_material_for_player(position.current_player()) as f32;
			let opponent_material = position.board().get_material_for_player(position.current_player().opposite()) as f32;

			let mobility_score = position.get_all_legal_moves().len() as f32 * 0.1;

			Evaluation::Score(own_material - opponent_material + mobility_score)
		}
	}
}

pub fn evaluate_material_and_mobility_i32(position: &Position) -> i32 {
	let own_material = position.board().get_material_for_player(position.current_player()) as i32;
	let opponent_material = position.board().get_material_for_player(position.current_player().opposite()) as i32;

	let mobility_score = position.get_all_legal_moves().len() as i32;

	return (own_material - opponent_material) * 100 + mobility_score;
}

pub fn evaluate_material_and_positioning(position: &Position) -> i32 {
	let own_material = position.board().get_material_for_player(position.current_player()) as i32;
	let opponent_material = position.board().get_material_for_player(position.current_player().opposite()) as i32;

	let positioning_score = position.get_positioning_score_for_player(position.current_player()) - position.get_positioning_score_for_player(position.current_player().opposite());

	return (own_material - opponent_material) * 100 + positioning_score;
}

pub fn evaluate_material_and_positioning_debug(position: &Position) -> (i32, i32, i32) {
	let own_material = position.board().get_material_for_player(position.current_player()) as i32;
	let opponent_material = position.board().get_material_for_player(position.current_player().opposite()) as i32;

	let positioning_score = position.get_positioning_score_for_player(position.current_player()) - position.get_positioning_score_for_player(position.current_player().opposite());

	return ((own_material - opponent_material) * 100 + positioning_score, own_material - opponent_material, positioning_score);
}

pub fn evaluate_material_and_positioning_by_phase(position: &Position) -> i32 {
	let game_phase = calculate_game_phase(position);

	let own_material = position.board().get_phase_material_for_player(position.current_player(), game_phase) as i32;
	let opponent_material = position.board().get_phase_material_for_player(position.current_player().opposite(), game_phase) as i32;

	let positioning_score = position.get_positioning_score_for_player_by_phase(position.current_player(), game_phase) - position.get_positioning_score_for_player_by_phase(position.current_player().opposite(), game_phase);

	return (own_material - opponent_material) + positioning_score;
}

pub fn evaluate_material_and_positioning_by_phase_with_bishop_pair(position: &Position) -> i32 {
	let game_phase = calculate_game_phase(position);

	let own_material = position.board().get_phase_material_for_player(position.current_player(), game_phase) as i32;
	let opponent_material = position.board().get_phase_material_for_player(position.current_player().opposite(), game_phase) as i32;

	let positioning_score = position.get_positioning_score_for_player_by_phase(position.current_player(), game_phase) - position.get_positioning_score_for_player_by_phase(position.current_player().opposite(), game_phase);
	let bishop_pair_bonus = bishop_pair_bonus(position, game_phase);

	return (own_material - opponent_material) + positioning_score + bishop_pair_bonus;
}

pub fn evaluate_phase_and_bishop_pair_and_rook_open_column(position: &Position) -> i32 {
	let game_phase = calculate_game_phase(position);

	let own_material = position.board().get_phase_material_for_player(position.current_player(), game_phase) as i32;
	let opponent_material = position.board().get_phase_material_for_player(position.current_player().opposite(), game_phase) as i32;

	let positioning_score = position.get_positioning_score_for_player_by_phase(position.current_player(), game_phase) - position.get_positioning_score_for_player_by_phase(position.current_player().opposite(), game_phase);
	
	let rook_open_column_bonus = rook_open_column_bonus(position);
	let bishop_pair_bonus = bishop_pair_bonus(position, game_phase);

	return (own_material - opponent_material) + positioning_score + bishop_pair_bonus + rook_open_column_bonus;
}