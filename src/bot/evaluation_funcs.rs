use crate::board::{game_state::GameState, position::Position};

use super::evaluation::Evaluation;

pub fn evaluate_material_only(position: &Position) -> Evaluation {
	let own_material = position.board().get_material_for_for_player(position.current_player()) as f32;
	let opponent_material = position.board().get_material_for_for_player(position.current_player().opposite()) as f32;

	Evaluation::Score(own_material - opponent_material)
}

pub fn evaluate_material_and_checkmates(position: &Position) -> Evaluation {
	match position.get_game_state() {
		GameState::Checkmate(winner) => if position.current_player() == winner { Evaluation::Checkmate(true) } else { Evaluation::Checkmate(false) },
		GameState::Stalemate => Evaluation::Stalemate,
		GameState::Ongoing => {
			let own_material = position.board().get_material_for_for_player(position.current_player()) as f32;
			let opponent_material = position.board().get_material_for_for_player(position.current_player().opposite()) as f32;

			Evaluation::Score(own_material - opponent_material)
		}
	}
}
