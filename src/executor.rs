use crate::board::{moove::Move, position::Position};

pub type EvaluationFn = fn(&Position) -> f32;
pub type SearchFn = fn(&Position, EvaluationFn) -> Move;

pub fn evaluate_material_only(position: &Position) -> f32 {
	let own_material = position.board().get_material_for_for_player(position.current_player()) as f32;
	let opponent_material = position.board().get_material_for_for_player(position.current_player().opposite()) as f32;

	own_material - opponent_material
}
