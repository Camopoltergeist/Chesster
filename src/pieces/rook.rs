pub struct Rook {
	player: Player
}

impl Rook {
	pub fn new(player: Player) -> Self {
		Self {
			player
		}
	}
}

impl Piece for Rook {
	fn piece_type(&self) -> PieceType {
		PieceType::Rook
	}

	fn player(&self) -> Player {
		self.player
	}

	fn get_movement_mask(&self, tile_pos: TilePosition) -> Bitboard {
		ROOK_MASKS[tile_pos.bit_offset() as usize]
	}
}