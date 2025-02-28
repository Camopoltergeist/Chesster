use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::{piece::PieceType, player::Player, player_piece::PlayerPiece};

use super::{moove::CastleSide, position::Position, tile_position::TilePosition};

pub struct ZobristHash {
	value: u64
}

impl ZobristHash {
	pub fn from_position(position: &Position) -> Self {
		let mut value = 0;

		for bit_offset in 0..64 {
			let tile_position = TilePosition::from_bit_offset(bit_offset);

			let piece = position.get_piece(tile_position);

			if piece.is_none() {
				continue;
			}

			let piece = piece.unwrap();

			value = value ^ get_zobrist_piece_number(piece, tile_position);
		};

		unsafe {
			if position.get_castling_availability(Player::White, CastleSide::KingSide) {
				value = value ^ WHITE_SHORT_CASTLE_NUMBER;
			};

			if position.get_castling_availability(Player::White, CastleSide::QueenSide) {
				value = value ^ WHITE_LONG_CASTLE_NUMBER;
			};

			if position.get_castling_availability(Player::Black, CastleSide::KingSide) {
				value = value ^ BLACK_SHORT_CASTLE_NUMBER;
			};

			if position.get_castling_availability(Player::Black, CastleSide::QueenSide) {
				value = value ^ BLACK_LONG_CASTLE_NUMBER;
			};

			if position.current_player() == Player::Black {
				value = value ^ BLACK_TO_MOVE;
			};

			if let Some(tile_pos) = position.en_passant_target.clone() {
				value = value ^ EN_PASSANT_COLUMN_NUMBERS[tile_pos.column() as usize];
			};
		};

		Self {
			value
		}
	}

	pub fn value(&self) -> u64 {
		self.value
	}
}

static mut WHITE_PAWN_NUMBERS: [u64; 64] = [0; 64];
static mut WHITE_ROOK_NUMBERS: [u64; 64] = [0; 64];
static mut WHITE_KNIGHT_NUMBERS: [u64; 64] = [0; 64];
static mut WHITE_BISHOP_NUMBERS: [u64; 64] = [0; 64];
static mut WHITE_QUEEN_NUMBERS: [u64; 64] = [0; 64];
static mut WHITE_KING_NUMBERS: [u64; 64] = [0; 64];

static mut BLACK_PAWN_NUMBERS: [u64; 64] = [0; 64];
static mut BLACK_ROOK_NUMBERS: [u64; 64] = [0; 64];
static mut BLACK_KNIGHT_NUMBERS: [u64; 64] = [0; 64];
static mut BLACK_BISHOP_NUMBERS: [u64; 64] = [0; 64];
static mut BLACK_QUEEN_NUMBERS: [u64; 64] = [0; 64];
static mut BLACK_KING_NUMBERS: [u64; 64] = [0; 64];

static mut WHITE_SHORT_CASTLE_NUMBER: u64 = 0;
static mut WHITE_LONG_CASTLE_NUMBER: u64 = 0;

static mut BLACK_SHORT_CASTLE_NUMBER: u64 = 0;
static mut BLACK_LONG_CASTLE_NUMBER: u64 = 0;

static mut BLACK_TO_MOVE: u64 = 0;

static mut EN_PASSANT_COLUMN_NUMBERS: [u64; 8] = [0; 8];

pub fn generate_zobrist_numbers() {
	let mut rng = ChaCha20Rng::seed_from_u64(1377);

	unsafe {
		WHITE_PAWN_NUMBERS = generate_numbers(&mut rng);
		WHITE_ROOK_NUMBERS = generate_numbers(&mut rng);
		WHITE_KNIGHT_NUMBERS = generate_numbers(&mut rng);
		WHITE_BISHOP_NUMBERS = generate_numbers(&mut rng);
		WHITE_QUEEN_NUMBERS = generate_numbers(&mut rng);
		WHITE_KING_NUMBERS = generate_numbers(&mut rng);

		BLACK_PAWN_NUMBERS = generate_numbers(&mut rng);
		BLACK_ROOK_NUMBERS = generate_numbers(&mut rng);
		BLACK_KNIGHT_NUMBERS = generate_numbers(&mut rng);
		BLACK_BISHOP_NUMBERS = generate_numbers(&mut rng);
		BLACK_QUEEN_NUMBERS = generate_numbers(&mut rng);
		BLACK_KING_NUMBERS = generate_numbers(&mut rng);

		WHITE_SHORT_CASTLE_NUMBER = rng.next_u64();
		WHITE_LONG_CASTLE_NUMBER = rng.next_u64();

		BLACK_SHORT_CASTLE_NUMBER = rng.next_u64();
		BLACK_LONG_CASTLE_NUMBER = rng.next_u64();

		BLACK_TO_MOVE = rng.next_u64();

		EN_PASSANT_COLUMN_NUMBERS = generate_en_passant_numbers(&mut rng);
	}
}

fn generate_numbers(rng: &mut ChaCha20Rng) -> [u64; 64] {
	let mut arr: [u64; 64] = [0; 64];

	for i in 0..64 {
		arr[i] = rng.next_u64();
	}

	return arr;
}

fn generate_en_passant_numbers(rng: &mut ChaCha20Rng) -> [u64; 8] {
	let mut arr: [u64; 8] = [0; 8];

	for i in 0..8 {
		arr[i] = rng.next_u64();
	}

	return arr;
}

fn get_zobrist_piece_number(piece: PlayerPiece, tile_position: TilePosition) -> u64 {
	let i = tile_position.bit_offset() as usize;

	unsafe {
		match piece.player() {
			Player::White => {
				match piece.piece() {
					PieceType::Pawn => WHITE_PAWN_NUMBERS[i],
					PieceType::Rook => WHITE_ROOK_NUMBERS[i],
					PieceType::Knight => WHITE_KNIGHT_NUMBERS[i],
					PieceType::Bishop => WHITE_BISHOP_NUMBERS[i],
					PieceType::Queen => WHITE_QUEEN_NUMBERS[i],
					PieceType::King => WHITE_KING_NUMBERS[i]
				}
			},
			Player::Black => {
				match piece.piece() {
					PieceType::Pawn => BLACK_PAWN_NUMBERS[i],
					PieceType::Rook => BLACK_ROOK_NUMBERS[i],
					PieceType::Knight => BLACK_KNIGHT_NUMBERS[i],
					PieceType::Bishop => BLACK_BISHOP_NUMBERS[i],
					PieceType::Queen => BLACK_QUEEN_NUMBERS[i],
					PieceType::King => BLACK_KING_NUMBERS[i]
				}
			}
		}
	}
}