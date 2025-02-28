use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub struct ZobristHash {
	value: u64
}

static mut ROOK_NUMBERS: [u64; 64] = [0; 64];
static mut KNIGHT_NUMBERS: [u64; 64] = [0; 64];
static mut BISHOP_NUMBERS: [u64; 64] = [0; 64];
static mut QUEEN_NUMBERS: [u64; 64] = [0; 64];
static mut KING_NUMBERS: [u64; 64] = [0; 64];
static mut PAWN_NUMBERS: [u64; 64] = [0; 64];

static mut WHITE_SHORT_CASTLE_NUMBER: u64 = 0;
static mut WHITE_LONG_CASTLE_NUMBER: u64 = 0;

static mut BLACK_SHORT_CASTLE_NUMBER: u64 = 0;
static mut BLACK_LONG_CASTLE_NUMBER: u64 = 0;

static mut BLACK_TO_MOVE: u64 = 0;

static mut EN_PASSANT_COLUMN_NUMBERS: [u64; 8] = [0; 8];

pub fn generate_zobrist_numbers() {
	let mut rng = ChaCha20Rng::seed_from_u64(1377);

	unsafe {
		ROOK_NUMBERS = generate_numbers(&mut rng);
		PAWN_NUMBERS = generate_numbers(&mut rng);
		KNIGHT_NUMBERS = generate_numbers(&mut rng);
		BISHOP_NUMBERS = generate_numbers(&mut rng);
		QUEEN_NUMBERS = generate_numbers(&mut rng);
		KING_NUMBERS = generate_numbers(&mut rng);

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