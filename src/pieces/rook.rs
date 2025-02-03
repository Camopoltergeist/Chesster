use crate::{board::{bitboard::Bitboard, move_mask::ROOK_MASKS, tile_position::TilePosition}, piece::{Piece, PieceType}, player::Player};

pub struct Rook {
	player: Player,
    tile_position: TilePosition,
}

impl Rook {
	pub fn new(player: Player, tile_position: TilePosition) -> Self {
		Self {
			player,
            tile_position,
		}
	}

    pub const fn get_movement_mask(tile_pos: TilePosition) -> Bitboard {
		ROOK_MASKS[tile_pos.bit_offset() as usize]
	}

    pub const fn generate_movement_mask(tile_pos: TilePosition) -> Bitboard {
        let rank_mask = Bitboard::generate_rank_mask(tile_pos.rank());
        let column_mask = Bitboard::get_column_mask(tile_pos.column());

        return Bitboard(rank_mask.0 ^ column_mask.0);
    }
}

impl Piece for Rook {
	fn piece_type(&self) -> PieceType {
		PieceType::Rook
	}

	fn player(&self) -> Player {
		self.player
	}

    fn movement_mask(&self) -> Bitboard {
        Self::get_movement_mask(self.tile_position)
    }
}