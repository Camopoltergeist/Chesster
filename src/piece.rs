#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Piece {
	Pawn,
	Rook,
	Knight,
	Bishop,
	Queen,
	King
}

impl Piece {
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