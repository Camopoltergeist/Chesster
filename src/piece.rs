//! Piece's type information.

use crate::{board::{bitboard::Bitboard, tile_position::TilePosition}, player::Player};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PieceType {
	Pawn,
	Rook,
	Knight,
	Bishop,
	Queen,
	King
}

impl PieceType {
	pub fn from_fen_char(c: char) -> Result<Self, ()> {
		let e = match c.to_ascii_lowercase() {
			'r' => Self::Rook,
			'n' => Self::Knight,
			'b' => Self::Bishop,
			'q' => Self::Queen,
			'k' => Self::King,
			'p' => Self::Pawn,
			_ => return Err(())
		};

		Ok(e)
	}
}

pub trait Piece {
	fn piece_type(&self) -> PieceType;
	fn player(&self) -> Player;
	fn tile_position(&self) -> TilePosition;

	fn movement_mask(&self) -> Bitboard;
}
