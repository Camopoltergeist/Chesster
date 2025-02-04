use std::collections::HashMap;

use raylib::{texture::{RaylibTexture2D, Texture2D}, RaylibHandle, RaylibThread};

use crate::{piece::PieceType, player::Player, player_piece::PlayerPiece};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PieceTexture {
	player: Player,
	piece: PieceType
}

impl PieceTexture {
	pub fn new(piece: PlayerPiece) -> Self {
		Self {
			player: piece.player(),
			piece: piece.piece()
		}
	}

	pub fn texture_string(&self) -> String {
		let player_str = if matches!(self.player, Player::Black) { "b" } else { "w" };

		let piece_str = match self.piece {
			PieceType::Pawn => "pawn",
			PieceType::Rook => "rook",
			PieceType::Knight => "knight",
			PieceType::Bishop => "bishop",
			PieceType::Queen => "queen",
			PieceType::King => "king"
		};

		return format!("{}_{}.png", player_str, piece_str).to_string();
	}
}

pub fn load_piece_textures(rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<PieceTexture, Texture2D> {
	let piece_textures = vec![
		PieceTexture::new(PlayerPiece::new(Player::White, PieceType::Pawn)),
		PieceTexture::new(PlayerPiece::new(Player::White, PieceType::Rook)),
		PieceTexture::new(PlayerPiece::new(Player::White, PieceType::Knight)),
		PieceTexture::new(PlayerPiece::new(Player::White, PieceType::Bishop)),
		PieceTexture::new(PlayerPiece::new(Player::White, PieceType::Queen)),
		PieceTexture::new(PlayerPiece::new(Player::White, PieceType::King)),

		PieceTexture::new(PlayerPiece::new(Player::Black, PieceType::Pawn)),
		PieceTexture::new(PlayerPiece::new(Player::Black, PieceType::Rook)),
		PieceTexture::new(PlayerPiece::new(Player::Black, PieceType::Knight)),
		PieceTexture::new(PlayerPiece::new(Player::Black, PieceType::Bishop)),
		PieceTexture::new(PlayerPiece::new(Player::Black, PieceType::Queen)),
		PieceTexture::new(PlayerPiece::new(Player::Black, PieceType::King)),
	];

	let texture_dir = "./res/pieces/";

	let mut texture_map = HashMap::new();

	for piece_texture in piece_textures {
		let file_path = format!("{}{}", texture_dir, piece_texture.texture_string());

		let mut texture = rl.load_texture(thread, &file_path).expect("failed to load piece texture");
		texture.set_texture_filter(thread, raylib::ffi::TextureFilter::TEXTURE_FILTER_TRILINEAR);
		texture.gen_texture_mipmaps();

		texture_map.insert(piece_texture, texture);
	};

	return texture_map;
}