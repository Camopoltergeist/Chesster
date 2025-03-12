use std::collections::{HashMap, HashSet};

use jja::{polyglot::to_move, polyglotbook::PolyGlotBook};
use shakmaty::{fen::Fen, zobrist::{Zobrist64, ZobristHash}, Chess, Position};

use crate::{board::{moove::{self, BasicMove, EnPassantMove, PromotingMove}, position, tile_position::TilePosition}, piece::PieceType, player::Player, player_piece::PlayerPiece};

pub fn load_opening_book() -> HashMap<u64, Vec<moove::Move>> {
	let old_book = PolyGlotBook::open("./baron30.bin").unwrap();

    let default_pos = shakmaty::Chess::new();
    let mut new_book: HashMap<u64, Vec<moove::Move>> = HashMap::new();
    let mut visited: HashSet<u64> = HashSet::new();

    convert(&default_pos, &old_book, &mut new_book, &mut visited);

	return new_book;
}

pub fn convert(position: &Chess, old_book: &PolyGlotBook, new_book: &mut HashMap<u64, Vec<moove::Move>>, visited: &mut HashSet<u64>) {
	let fen_string = Fen::from_position(position.clone(), shakmaty::EnPassantMode::Always).to_string();
	let old_hash: u64 = position.zobrist_hash::<Zobrist64>(shakmaty::EnPassantMode::PseudoLegal).into();

	if visited.contains(&old_hash) {
		return;
	}

	visited.insert(old_hash);

	let our_position = position::Position::from_fen_str(&fen_string).unwrap();

	let new_hash = our_position.hash().value();

	let mut new_entry = Vec::new();


	if let Some(entries) = old_book.lookup_moves(old_hash) {
		for e in entries {
			if let Some(s_move) = to_move(position, e.mov) {
				let our_move = s_move_to_our_move(&s_move, our_position.current_player());
				
				new_entry.push(our_move);
				
				let moved_position = position.clone().play(&s_move).unwrap();
				
				convert(&moved_position, old_book, new_book, visited);
			}
		}
	}

	if new_entry.is_empty() {
		return;
	}

	new_book.insert(new_hash, new_entry);
}

fn s_move_to_our_move(s_move: &shakmaty::Move, player: Player) -> moove::Move {
	if s_move.is_castle() {
		return match s_move.castling_side().unwrap() {
			shakmaty::CastlingSide::KingSide => moove::Move::new_castling(player, moove::CastleSide::KingSide),
			shakmaty::CastlingSide::QueenSide => moove::Move::new_castling(player, moove::CastleSide::QueenSide)
		}
	};

	let from_square = s_move.from().unwrap();
	let to_square = s_move.to();

	let from = TilePosition::new(from_square.file().into(), from_square.rank().into());
	let to = TilePosition::new(to_square.file().into(), to_square.rank().into());

	if s_move.is_promotion() {
		let piece_type = match s_move.promotion().unwrap() {
			shakmaty::Role::Queen => PieceType::Queen,
			shakmaty::Role::Rook => PieceType::Rook,
			shakmaty::Role::Bishop => PieceType::Bishop,
			shakmaty::Role::Knight => PieceType::Knight,
			_ => unreachable!("Invalid promotion piece!")
		};

		let player_piece = PlayerPiece::new(player, piece_type);

		return moove::Move::Promoting(PromotingMove::new(from, to, player_piece));
	};

	if s_move.is_en_passant() {
		let captured_tile = TilePosition::new(from.rank(), to.column());

		return moove::Move::EnPassant(EnPassantMove::new(from, to, captured_tile));
	};

	return moove::Move::Basic(BasicMove::new(from, to));
}