use crate::board::{moove::Move, position::Position};

pub fn perft(position: &Position, depth: u32) -> Vec<(Move, u64)> {
	fn negamax(position: &Position, depth: u32) -> u64 {
		if depth < 1 {
			return 1;
		};

		let legal_moves = position.get_all_legal_moves();

		if legal_moves.len() < 1 {
			return 0;
		}

        let mut searched_positions = 0;

		for m in legal_moves {
			let mut moved_position = position.clone();
			moved_position.make_move(m.clone());

			searched_positions += negamax(&moved_position, depth - 1);
		};

        return searched_positions;
	}

	if depth < 1 {
		return Vec::new();
	};

	let legal_moves = position.get_all_legal_moves();

	if legal_moves.len() < 1 {
		return Vec::new();
	};

	let mut moves = Vec::new();
	
	for m in legal_moves {
		let mut moved_position = position.clone();
		moved_position.make_move(m.clone());
		
		let searched_positions = negamax(&moved_position, depth - 1);
		
        moves.push((m, searched_positions));
	};

	return moves;
}