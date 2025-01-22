use std::collections::HashMap;

use raylib::{texture::{RaylibTexture2D, Texture2D}, RaylibHandle, RaylibThread};

use crate::{piece::Piece, player::Player};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PieceTexture {
	player: Player,
	piece: Piece
}

impl PieceTexture {
	pub fn new(player: Player, piece: Piece) -> Self {
		Self {
			player,
			piece
		}
	}

	pub fn texture_string(&self) -> String {
		let player_str = if matches!(self.player, Player::Black) { "b" } else { "w" };

		let piece_str = match self.piece {
			Piece::Pawn => "pawn",
			Piece::Rook => "rook",
			Piece::Knight => "knight",
			Piece::Bishop => "bishop",
			Piece::Queen => "queen",
			Piece::King => "king"
		};

		return format!("{}_{}.png", player_str, piece_str).to_string();
	}
}

pub fn load_piece_textures(rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<PieceTexture, Texture2D> {
	let piece_textures = vec![
		PieceTexture::new(Player::White, Piece::Pawn),
		PieceTexture::new(Player::White, Piece::Rook),
		PieceTexture::new(Player::White, Piece::Knight),
		PieceTexture::new(Player::White, Piece::Bishop),
		PieceTexture::new(Player::White, Piece::Queen),
		PieceTexture::new(Player::White, Piece::King),

		PieceTexture::new(Player::Black, Piece::Pawn),
		PieceTexture::new(Player::Black, Piece::Rook),
		PieceTexture::new(Player::Black, Piece::Knight),
		PieceTexture::new(Player::Black, Piece::Bishop),
		PieceTexture::new(Player::Black, Piece::Queen),
		PieceTexture::new(Player::Black, Piece::King),
	];

	let texture_dir = "./res/pieces/";

	let mut texture_map = HashMap::new();

	for piece_texture in piece_textures {
		let file_path = format!("{}{}", texture_dir, piece_texture.texture_string());

		let mut texture = rl.load_texture(thread, &file_path).expect("failed to load piece texture");
		texture.gen_texture_mipmaps();

		texture_map.insert(piece_texture, texture);
	};

	return texture_map;
}