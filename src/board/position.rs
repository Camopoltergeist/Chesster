//! A module for handling game position and legal moves.

use std::hash::Hash;

use crate::{board::moove::CastleSide, bot::{positioning::get_score_for_piece, utils::calculate_game_phase}, piece::PieceType, pieces::{king::King, pawn::Pawn}, player::Player, player_piece::PlayerPiece};

use super::{board::Board, game_state::GameState, moove::{BasicMove, CastlingMove, EnPassantMove, Move, PromotingMove}, move_collision::get_collision_mask, tile_position::TilePosition, zobrist_hash::ZobristHash};

/// Represents an entire chess position.
#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    board: Board,
    current_player: Player,

    pub en_passant_target: Option<TilePosition>,

    white_short_castling: bool,
    white_long_castling: bool,
    black_short_castling: bool,
    black_long_castling: bool,

    zobrist_hash: ZobristHash,
}

impl Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.board.get_all_pieces_mask().hash(state);
    }
}

impl Position {
    /// [`Player`] who's turn it is.
    pub fn current_player(&self) -> Player {
        self.current_player
    }

    /// Reference to contained [`Board`]
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Construct a position from a FEN-notation `&str`.
    pub fn from_fen_str(fen: &str) -> Result<Self, FenParseError> {
        let mut board = Board::empty();

        let mut column = 0;
        let mut rank = 7;

        let split: Vec<&str> = fen.split(' ').collect();

        if split.len() < 6 {
            return Err(FenParseError::UnexpectedEnd);
        }

        let pieces_str = split[0];
        let player_str = split[1];
        let castling_str = split[2];
        let en_passant_target_str = split[3];
        let _half_move_clock = split[4];
        let _full_move_clock = split[5];

        for fen_char in pieces_str.chars() {
            if fen_char.is_numeric() {
                column += fen_char.to_digit(10).unwrap();

                if column > 8 {
                    return Err(FenParseError::OutOfBoard);
                };

                continue;
            }

            if fen_char == '/' {
                column = 0;
                rank -= 1;
                continue;
            }

            if rank > 7 || column > 7 {
                return Err(FenParseError::OutOfBoard);
            }

            let player = Player::from_fen_piece_char(fen_char);
            let piece_result = PieceType::from_fen_char(fen_char);

            if piece_result.is_err() {
                return Err(FenParseError::InvalidPiece);
            }

            let piece = piece_result.unwrap();

            board.set_piece(PlayerPiece::new(player, piece), TilePosition::new(column, rank));
            column += 1;
        };

        if player_str.len() > 1 {
            return Err(FenParseError::InvalidPlayer);
        }

        let current_player = Player::from_fen_char(player_str.chars().nth(0).unwrap());

        let mut white_short_castling = false;
        let mut white_long_castling = false;
        let mut black_short_castling = false;
        let mut black_long_castling = false;

        if !castling_str.starts_with("-") {
            for char in castling_str.chars() {
                match char {
                    'K' => white_short_castling = true,
                    'Q' => white_long_castling = true,
                    'k' => black_short_castling = true,
                    'q' => black_long_castling = true,
                    _ => return Err(FenParseError::InvalidCastlingChar)
                }
            }
        }

        let en_passant_target: Option<TilePosition> = match en_passant_target_str {
            "-" => None,
            _ => Some(TilePosition::from_tile_str(en_passant_target_str).unwrap())
        };

        let mut s = Self{
            board,
            current_player,
            white_short_castling,
            white_long_castling,
            black_short_castling,
            black_long_castling,
            en_passant_target,
            ..Default::default()
        };

        s.generate_zobrist_hash();

        Ok(s)
    }

    /// Print all found legal moves.
    pub fn print_all_legal_moves(&self) {
        let mut counter = 1;

		for m in self.get_all_legal_moves() {
			println!("{}: {}", counter, m.debug_string());
			counter += 1;
		}
    }

    /// Print all castling availabilities.
    pub fn print_castling_availability(&self) {
        println!("Castling available:");
        println!("White King side: {}", self.white_short_castling);
        println!("White Queen side: {}", self.white_long_castling);
        println!("Black King side: {}", self.black_short_castling);
        println!("Black Queen side: {}", self.black_long_castling);
    }

    /// Return `true` if current position contains a check against [`player`][Player].
    pub fn is_in_check(&self, player: Player) -> bool {
        let king_mask = *self.board.get_piece_bitboard(PieceType::King) & *self.board.get_player_bitboard(player);
        let attack_mask = self.board.get_attack_mask(player.opposite());

        return !(king_mask & attack_mask).is_empty();
    }

    pub fn get_game_state(&self) -> GameState {
        let legal_moves = self.get_all_legal_moves();

        if legal_moves.len() > 0 {
            return GameState::Ongoing;
        };

        if self.is_in_check(self.current_player) {
            return GameState::Checkmate(self.current_player.opposite());
        };

        return GameState::Stalemate;
    }

    /// Generate all legal moves and return them.
    pub fn get_all_legal_moves(&self) -> Vec<Move> {
        let piece_mask = self.board.get_player_bitboard(self.current_player);

        let mut legal_moves = Vec::new();

        for bit_offset in 0..64 {
            if !piece_mask.check_bit(bit_offset) {
                continue;
            }

            let tile_pos = TilePosition::from_bit_offset(bit_offset);

            legal_moves.extend(self.generate_legal_moves_for_tile_position(tile_pos));
        };

        return legal_moves;
    }

    pub fn generate_legal_moves_for_tile_position(&self, tile_pos: TilePosition) -> Vec<Move> {
        let piece = self.get_piece(tile_pos);

        if piece.is_none() {
            return Vec::new();
        };

        let piece = piece.unwrap();

        match piece.piece() {
            PieceType::Pawn => self.generate_legal_pawn_moves(tile_pos, piece.player()),
            PieceType::King => self.generate_legal_king_moves(tile_pos, piece.player()),
            _ => self.generate_legal_basic_moves(tile_pos)
        }
    }

    pub fn generate_legal_basic_moves(&self, tile_pos: TilePosition) -> Vec<Move> {
        let mut collision_mask = get_collision_mask(self.board.clone(), tile_pos);

        let mut moves: Vec<Move> = Vec::new();

        while !collision_mask.is_empty() {
            let bit_offset = collision_mask.0.trailing_zeros();

            let to_pos = TilePosition::from_bit_offset(bit_offset);

            let basic_move: Move = BasicMove::new(tile_pos, to_pos).into();

            if !self.does_move_leave_king_threatened(&basic_move) {
                moves.push(basic_move);
            }

            collision_mask.unset_bit(bit_offset);
        }

        return moves;
    }

    pub fn generate_legal_pawn_moves(&self, tile_pos: TilePosition, player: Player) -> Vec<Move> {
        let collision_mask = Pawn::generate_collision_mask(&self.board, player, tile_pos);

        let mut moves: Vec<Move> = Vec::new();

        if self.can_en_passant(tile_pos) {
            let target = self.en_passant_target.unwrap();
            let en_passant_move: Move = EnPassantMove::new(tile_pos, target, TilePosition::new(target.column(), tile_pos.rank())).into();

            if !self.does_move_leave_king_threatened(&en_passant_move) {
                moves.push(en_passant_move);
            };
        };

        for bit_offset in 0..64 {
            if !collision_mask.check_bit(bit_offset) {
                continue;
            }

            let to_pos = TilePosition::from_bit_offset(bit_offset);

            if self.can_promote(tile_pos, to_pos) {
                let queen_promoting_move: Move = PromotingMove::new(tile_pos, to_pos, PlayerPiece::new(player, PieceType::Queen)).into();

                if self.is_legal_move(&queen_promoting_move) {
                    moves.push(queen_promoting_move);
                    moves.push(PromotingMove::new(tile_pos, to_pos, PlayerPiece::new(player, PieceType::Knight)).into());
                    moves.push(PromotingMove::new(tile_pos, to_pos, PlayerPiece::new(player, PieceType::Rook)).into());
                    moves.push(PromotingMove::new(tile_pos, to_pos, PlayerPiece::new(player, PieceType::Bishop)).into());
                }

                continue;
            }

            let basic_move: Move = BasicMove::new(tile_pos, to_pos).into();

            if !self.does_move_leave_king_threatened(&basic_move) {
                moves.push(basic_move);
            }
        };

        return moves;
    }

    pub fn generate_legal_king_moves(&self, tile_pos: TilePosition, player: Player) -> Vec<Move> {
        let collision_mask = King::generate_collision_mask(&self.board, player, tile_pos);

        let mut moves: Vec<Move> = self.get_legal_castling_moves();

        for bit_offset in 0..64 {
            if !collision_mask.check_bit(bit_offset) {
                continue;
            }

            let to_pos = TilePosition::from_bit_offset(bit_offset);

            let basic_move: Move = BasicMove::new(tile_pos, to_pos).into();

            if !self.does_move_leave_king_threatened(&basic_move) {
                moves.push(basic_move);
            }
        };

        return moves;
    }

    fn can_promote(&self, from_pos: TilePosition, to_pos: TilePosition) -> bool {
        if !self.board.check_for_pawn(from_pos) {
            return false;
        };

        let promotable_rank = match self.current_player {
            Player::White => 7,
            Player::Black => 0
        };

        return to_pos.rank() == promotable_rank;
    }

    fn can_en_passant(&self, tile_pos: TilePosition) -> bool {
        if self.en_passant_target.is_none() {
            return false;
        }

        if !self.board.check_for_pawn(tile_pos) {
            return false;
        }

        let en_passant_target = self.en_passant_target.unwrap();

        if let Some(left_capture) = en_passant_target.get_en_passant_left_capture(self.current_player) {
            if left_capture == tile_pos {
                return true;
            }
        }

        if let Some(right_capture) = en_passant_target.get_en_passant_right_capture(self.current_player) {
            if right_capture == tile_pos {
                return true;
            }
        }

        return false;
    }

    pub fn get_legal_castling_moves(&self) -> Vec<Move> {
        if self.is_in_check(self.current_player) {
            return Vec::new();
        }

        let mut castling_moves = Vec::with_capacity(4);
        
        match self.current_player {
            Player::White => {
                if self.white_short_castling {
                    if let Some(moove) = self.get_castling_move_if_legal(Player::White, CastleSide::KingSide) {
                        castling_moves.push(moove);
                    }
                }
        
                if self.white_long_castling {
                    if let Some(moove) = self.get_castling_move_if_legal(Player::White, CastleSide::QueenSide) {
                        castling_moves.push(moove);
                    }
                }
            },
            Player::Black => {
                if self.black_short_castling {
                    if let Some(moove) = self.get_castling_move_if_legal(Player::Black, CastleSide::KingSide) {
                        castling_moves.push(moove);
                    }
                }
        
                if self.black_long_castling {
                    if let Some(moove) = self.get_castling_move_if_legal(Player::Black, CastleSide::QueenSide) {
                        castling_moves.push(moove);
                    }
                }
            }
        };

        return castling_moves;
    }

    pub fn get_castling_move_if_legal(&self, player: Player, side: CastleSide) -> Option<Move> {
        if self.board.is_castling_possible(player, side.clone()) {
            return Some(CastlingMove::new(player, side).into());
        };

        return None;
    }

    /// Get [`PlayerPiece`] at [`tile_pos`][TilePosition].
    /// 
    /// Returns [`None`] if no piece is located at the position.
    /// 
    /// [`None`]: Option#variant.None
    pub fn get_piece(&self, tile_pos: TilePosition) -> Option<PlayerPiece> {
        self.board.get_piece(tile_pos)
    }

    pub fn get_piece_debug(&self, tile_str: &str) -> Option<PlayerPiece> {
        self.board.get_piece_debug(tile_str)
    }

    pub fn does_move_leave_king_threatened(&self, moove: &Move) -> bool {
        let mut moved_position = self.clone();
        moved_position.make_move_unchecked(moove.clone());

        return moved_position.is_in_check(self.current_player);
    }

    pub fn is_legal_move(&self, moove: &Move) -> bool {
        let base_legal = match moove {
            Move::Basic(basic_move) => self.is_legal_basic_move(basic_move),
            Move::Castling(castling_move) => self.is_legal_castling_move(castling_move),
            Move::EnPassant(en_passant_move) => self.is_legal_en_passant_move(en_passant_move),
            Move::Promoting(promoting_move) => self.is_legal_promoting_move(promoting_move),
        };

        let leaves_king_threatened = self.does_move_leave_king_threatened(moove);

        return base_legal && !leaves_king_threatened;
    }

    fn is_legal_basic_move(&self, basic_move: &BasicMove) -> bool {
        let collision_mask = get_collision_mask(self.board.clone(), basic_move.from_position());
        if !collision_mask.check_bit(basic_move.to_position().bit_offset()) {
            return false;
        };

        let piece = self.board.get_piece(basic_move.from_position()).unwrap();

        if piece.player() != self.current_player {
            return false;
        };

        return true;
    }

    fn is_legal_castling_move(&self, castling_move: &CastlingMove) -> bool {
        if self.is_in_check(self.current_player) {
            return false;
        }
        
        self.board.is_castling_possible(castling_move.player(), castling_move.side())
    }

    fn is_legal_en_passant_move(&self, en_passant_move: &EnPassantMove) -> bool {
        self.can_en_passant(en_passant_move.from_position())
    }

    fn is_legal_promoting_move(&self, promoting_move: &PromotingMove) -> bool {
        return self.is_legal_basic_move(&promoting_move.clone().into()) && promoting_move.promotion_piece().player() == self.current_player;
    }

    /// Makes a chess move and passes turn to other player.
    pub fn make_move(&mut self, moove: Move) {
        debug_assert!(self.is_legal_move(&moove));

        self.make_move_unchecked(moove);
    }

    /// Makes a chess move and passes turn to other player.
    /// 
    /// This move does not check move legality.
    pub fn make_move_unchecked(&mut self, moove: Move) {
        self.change_castling_availability_if_needed(&moove);

        if let Some(en_passant_tile) = self.en_passant_target.clone() {
            self.zobrist_hash.update_en_passant_column(en_passant_tile);
        }

        self.en_passant_target = self.get_en_passant_target_for_move(&moove);

        if let Some(en_passant_tile) = self.en_passant_target.clone() {
            self.zobrist_hash.update_en_passant_column(en_passant_tile);
        }

        match moove {
            Move::Basic(basic_move) => {
                let moved_piece = self.get_piece(basic_move.from_position()).unwrap();
                let captured_piece = self.get_piece(basic_move.to_position());

                self.zobrist_hash.update_basic_move(basic_move.clone(), moved_piece, captured_piece);
                self.board.move_piece_basic(basic_move);
            },
            Move::Castling(castling_move) => {
                let moved_piece = self.get_piece(castling_move.from_position()).unwrap();

                self.zobrist_hash.update_castling_move(castling_move.clone(), moved_piece);
                self.board.move_piece_castling(castling_move);
            },
            Move::EnPassant(en_passant_move) => {
                let moved_piece = self.get_piece(en_passant_move.from_position()).unwrap();
                let captured_piece = self.get_piece(en_passant_move.captured_tile()).unwrap();

                self.zobrist_hash.update_en_passant_move(en_passant_move.clone(), moved_piece, captured_piece);
                self.board.move_piece_en_passant(en_passant_move)
            },
            Move::Promoting(promoting_move) => {
                let moved_piece = self.get_piece(promoting_move.from_position()).unwrap();
                let captured_piece = self.get_piece(promoting_move.to_position());

                self.zobrist_hash.update_promoting_move(promoting_move.clone(), moved_piece, captured_piece);
                self.board.move_piece_promoting(promoting_move);
            },
        }

        self.current_player = self.current_player.opposite();
    }

    /// [`ZobristHash`] for current position.
    pub fn hash(&self) -> &ZobristHash {
        &self.zobrist_hash
    }

    fn get_en_passant_target_for_move(&self, moove: &Move) -> Option<TilePosition> {
        let from_pos = moove.from_position();

        if !self.board.check_for_pawn(from_pos) {
            return None;
        };

        let to_pos = moove.to_position();

        let move_length: i32 = from_pos.rank() as i32 - to_pos.rank() as i32;

        if move_length.abs() != 2 {
            return None;
        };

        return match self.current_player {
            Player::White => Some(TilePosition::new(to_pos.column(), 2)),
            Player::Black => Some(TilePosition::new(to_pos.column(), 5))
        }
    }

    fn change_castling_availability_if_needed(&mut self, moove: &Move) {
        let from_pos = moove.from_position();
        let piece = self.board.get_piece(from_pos).expect("no piece at position");

        // Handle king moves
        if PieceType::King == piece.piece() {
            match self.current_player {
                Player::White => {
                    if self.white_short_castling {
                        self.white_short_castling = false;
                        self.zobrist_hash.update_castling_availability(Player::White, CastleSide::KingSide);
                    }

                    if self.white_long_castling {
                        self.white_long_castling = false;
                        self.zobrist_hash.update_castling_availability(Player::White, CastleSide::QueenSide);
                    }
                },
                Player::Black => {
                    if self.black_short_castling {
                       self.black_short_castling = false;
                       self.zobrist_hash.update_castling_availability(Player::Black, CastleSide::KingSide);
                    }

                    if self.black_long_castling {
                        self.black_long_castling = false;
                        self.zobrist_hash.update_castling_availability(Player::Black, CastleSide::QueenSide);
                    }
                }
            }

            return;
        }

        // Handle own rook moves
        if PieceType::Rook == piece.piece() {
            match piece.player() {
                Player::White => {
                    if from_pos == TilePosition::new(0, 0) {
                        if self.white_long_castling {
                            self.white_long_castling = false;
                            self.zobrist_hash.update_castling_availability(Player::White, CastleSide::QueenSide);
                        }
                    }
                    else if from_pos == TilePosition::new(7, 0) {
                        if self.white_short_castling {
                            self.white_short_castling = false;
                            self.zobrist_hash.update_castling_availability(Player::White, CastleSide::KingSide);
                        }
                    }
                },
                Player::Black => {
                    if from_pos == TilePosition::new(0, 7) {
                        if self.black_long_castling {
                            self.black_long_castling = false;
                            self.zobrist_hash.update_castling_availability(Player::Black, CastleSide::QueenSide);
                        }
                    }
                    else if from_pos == TilePosition::new(7, 7) {
                        if self.black_short_castling {
                            self.black_short_castling = false;
                            self.zobrist_hash.update_castling_availability(Player::Black, CastleSide::KingSide);
                        }
                    }
                }
            }
        }

        // Handle capturing moves for opponent's castling
        let to_pos = moove.to_position();

        if to_pos == TilePosition::new(0, 0) {
            if self.white_long_castling {
                self.white_long_castling = false;
                self.zobrist_hash.update_castling_availability(Player::White, CastleSide::QueenSide);
            }
            return;
        }

        if to_pos == TilePosition::new(7, 0) {
            if self.white_short_castling {
                self.white_short_castling = false;
                self.zobrist_hash.update_castling_availability(Player::White, CastleSide::KingSide);
            }
            return;
        }

        if to_pos == TilePosition::new(0, 7) {
            if self.black_long_castling {
                self.black_long_castling = false;
                self.zobrist_hash.update_castling_availability(Player::Black, CastleSide::QueenSide);
            }
            return;
        }

        if to_pos == TilePosition::new(7, 7) {
            if self.black_short_castling {
                self.black_short_castling = false;
                self.zobrist_hash.update_castling_availability(Player::Black, CastleSide::KingSide);
            }
            return;
        }
    }

    /// This function returns true if tile_position contains specified piece or nothing if None was passed.
    pub fn debug_check_tile(&self, tile_str: &str, expected_piece: Option<(Player, PieceType)>) -> bool {
        let expected_piece = if let Some(p) = expected_piece {
            Some(PlayerPiece::new(p.0, p.1))
        }
        else {
            None
        };

        let piece_on_board = self.get_piece(TilePosition::from_tile_str(tile_str).unwrap());

        return piece_on_board == expected_piece;
    }

    pub fn get_castling_availability(&self, player: Player, side: CastleSide) -> bool {
        match (player, side) {
            (Player::White, CastleSide::KingSide) => self.white_short_castling,
            (Player::White, CastleSide::QueenSide) => self.white_long_castling,
            (Player::Black, CastleSide::KingSide) => self.black_short_castling,
            (Player::Black, CastleSide::QueenSide) => self.black_long_castling
        }
    }

    pub fn get_positioning_score_for_player(&self, player: Player) -> i32 {
        let mut player_pieces_mask = self.board.get_player_bitboard(player).clone();

        let mut positioning_score = 0;

        let game_phase = calculate_game_phase(self);

        while !player_pieces_mask.is_empty() {
            let bit_offset = player_pieces_mask.0.trailing_zeros();

            let tile_position = TilePosition::from_bit_offset(bit_offset);
            let piece = self.get_piece(tile_position).unwrap();

            positioning_score += get_score_for_piece(piece, tile_position, game_phase, self);

            player_pieces_mask.unset_bit(bit_offset);
        }

        return positioning_score;
    }

    pub fn get_positioning_score_for_player_by_phase(&self, player: Player, game_phase: (i32, i32)) -> i32 {
        let mut player_pieces_mask = self.board.get_player_bitboard(player).clone();

        let mut positioning_score = 0;

        while !player_pieces_mask.is_empty() {
            let bit_offset = player_pieces_mask.0.trailing_zeros();

            let tile_position = TilePosition::from_bit_offset(bit_offset);
            let piece = self.get_piece(tile_position).unwrap();

            positioning_score += get_score_for_piece(piece, tile_position, game_phase, self);

            player_pieces_mask.unset_bit(bit_offset);
        }

        return positioning_score;
    }

    pub fn generate_zobrist_hash(&mut self) {
        self.zobrist_hash = ZobristHash::from_position(&self);
    }
}

impl Position {
    pub fn new(board: Board, current_player: Player) -> Self {
        Self {
            board,
            current_player,
            ..Default::default()
        }
    }

    pub fn empty() -> Self {
        Self {
            board: Board::empty(),
            white_short_castling: false,
            white_long_castling: false,
            black_short_castling: false,
            black_long_castling: false,
            ..Default::default()
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        let mut s = Self {
            board: Board::default(),
            current_player: Player::White,

            en_passant_target: None,
            white_short_castling: true,
            white_long_castling: true,
            black_short_castling: true,
            black_long_castling: true,

            zobrist_hash: ZobristHash::zero()
        };

        s.generate_zobrist_hash();

        s
    }
}

#[derive(Debug)]
pub enum FenParseError {
    InvalidPiece,
    InvalidPlayer,
    InvalidCastlingChar,
    OutOfBoard,
    UnexpectedEnd,
}