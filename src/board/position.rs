use crate::{board::moove::CastleSide, piece::PieceType, player::Player, player_piece::PlayerPiece};

use super::{board::Board, moove::{BasicMove, CastlingMove, EnPassantMove, Move, PromotingMove}, move_collision::get_collision_mask, tile_position::TilePosition};

#[derive(Clone)]
pub struct Position {
    board: Board,
    current_player: Player,

    pub en_passant_target: Option<TilePosition>,

    white_short_castling: bool,
    white_long_castling: bool,
    black_short_castling: bool,
    black_long_castling: bool,
}

impl Position {
    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

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
        let en_passant_target = split[3];
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

        Ok(Self{
            board,
            current_player,
            white_short_castling,
            white_long_castling,
            black_short_castling,
            black_long_castling,
            ..Default::default()
        })
    }

    pub fn print_all_legal_moves(&self) {
        let mut counter = 1;

		for m in self.get_all_legal_moves() {
			println!("{}: {}", counter, m.debug_string());
			counter += 1;
		}
    }

    pub fn print_castling_availability(&self) {
        println!("Castling available:");
        println!("White King side: {}", self.white_short_castling);
        println!("White Queen side: {}", self.white_long_castling);
        println!("Black King side: {}", self.black_short_castling);
        println!("Black Queen side: {}", self.black_long_castling);
    }

    pub fn is_in_check(&self, player: Player) -> bool {
        let king_mask = *self.board.get_piece_bitboard(PieceType::King) & *self.board.get_player_bitboard(player);
        let attack_mask = self.board.get_attack_mask(player.opposite());

        return !(king_mask & attack_mask).is_empty();
    }

    pub fn get_all_legal_moves(&self) -> Vec<Move> {
        let piece_mask = self.board.get_player_bitboard(self.current_player);

        let mut legal_moves = Vec::new();

        for bit_offset in 0..64 {
            if !piece_mask.check_bit(bit_offset) {
                continue;
            }

            let tile_pos = TilePosition::from_bit_offset(bit_offset);

            legal_moves.append(&mut self.get_legal_moves_for_tile_position(tile_pos));
        };

        return legal_moves;
    }

    pub fn get_legal_moves_for_tile_position(&self, tile_pos: TilePosition) -> Vec<Move> {
        let mut moves = Vec::new();

        if let Some(piece) = self.board.get_piece(tile_pos) {
            let moves_bitboard = get_collision_mask(self.board.clone(), tile_pos);

            if moves_bitboard.is_empty() {
                return moves;
            }

            for bit_offset in 0..64 {
                if moves_bitboard.check_bit(bit_offset) {
                    let to_pos = TilePosition::from_bit_offset(bit_offset);

                    if self.can_promote(tile_pos, to_pos) {
                        moves.push(PromotingMove::new(tile_pos, to_pos, PlayerPiece::new(self.current_player, PieceType::Queen)).into());
                        moves.push(PromotingMove::new(tile_pos, to_pos, PlayerPiece::new(self.current_player, PieceType::Knight)).into());
                        moves.push(PromotingMove::new(tile_pos, to_pos, PlayerPiece::new(self.current_player, PieceType::Rook)).into());
                        moves.push(PromotingMove::new(tile_pos, to_pos, PlayerPiece::new(self.current_player, PieceType::Bishop)).into());
                    }
                    else {
                        moves.push(BasicMove::new(tile_pos, TilePosition::from_bit_offset(bit_offset)).into());
                    }
                }
            }

            match piece.piece() {
                PieceType::King => {
                    let castling_moves = self.get_legal_castling_moves();
                    moves.extend(castling_moves);
                },
                PieceType::Pawn => {
                    if self.can_en_passant(tile_pos) {
                        let target = self.en_passant_target.unwrap();

                        moves.push(EnPassantMove::new(tile_pos, target, TilePosition::new(target.column(), tile_pos.rank())).into());
                    }
                }
                _ => ()
            }
        }

        let mut legal_moves = Vec::new();

        for m in moves {
            let mut moved_position = self.clone();
            moved_position.make_move(m.clone()).expect("unexpected illegal move while culling moves");
            
            if !moved_position.is_in_check(self.current_player) {
                legal_moves.push(m);
            }
        }

        return legal_moves;
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

    pub fn get_piece(&self, tile_pos: TilePosition) -> Option<PlayerPiece> {
        self.board.get_piece(tile_pos)
    } 

    pub fn is_legal_move(&self, moove: &Move) -> bool {
        match moove {
            Move::Basic(basic_move) => self.is_legal_basic_move(basic_move),
            Move::Castling(castling_move) => self.is_legal_castling_move(castling_move),
            Move::EnPassant(en_passant_move) => self.is_legal_en_passant_move(en_passant_move),
            Move::Promoting(promoting_move) => self.is_legal_promoting_move(promoting_move),
        }
    }

    /// Checks if a BasicMove is legal
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
        self.board.is_castling_possible(castling_move.player(), castling_move.side())
    }

    fn is_legal_en_passant_move(&self, en_passant_move: &EnPassantMove) -> bool {
        self.can_en_passant(en_passant_move.from_position())
    }

    fn is_legal_promoting_move(&self, promoting_move: &PromotingMove) -> bool {
        return self.is_legal_basic_move(&promoting_move.clone().into()) && promoting_move.promotion_piece().player() == self.current_player;
    }

    pub fn make_move(&mut self, moove: Move) -> Result<(), ()> {
        if !self.is_legal_move(&moove) {
            return Err(());
        };

        self.change_castling_availability_if_needed(&moove);

        self.en_passant_target = self.get_en_passant_target_for_move(&moove);

        match moove {
            Move::Basic(basic_move) => self.board.move_piece_basic(basic_move),
            Move::Castling(castling_move) => self.board.move_piece_castling(castling_move),
            Move::EnPassant(en_passant_move) => self.board.move_piece_en_passant(en_passant_move),
            Move::Promoting(promoting_move) => self.board.move_piece_promoting(promoting_move),
        }

        self.current_player = self.current_player.opposite();

        Ok(())
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
                    self.white_short_castling = false;
                    self.white_long_castling = false;
                },
                Player::Black => {
                    self.black_short_castling = false;
                    self.black_long_castling = false;
                }
            }

            return;
        }

        // Handle own rook moves
        if PieceType::Rook == piece.piece() {
            match piece.player() {
                Player::White => {
                    if from_pos == TilePosition::new(0, 0) {
                        self.white_long_castling = false;
                        return;
                    }

                    if from_pos == TilePosition::new(7, 0) {
                        self.white_short_castling = false;
                        return;
                    }
                },
                Player::Black => {
                    if from_pos == TilePosition::new(0, 7) {
                        self.black_long_castling = false;
                        return;
                    }

                    if from_pos == TilePosition::new(7, 7) {
                        self.black_short_castling = false;
                        return;
                    }
                }
            }

            return;
        }

        // Handle capturing moves for opponent's castling
        let to_pos = moove.to_position();

        if to_pos == TilePosition::new(0, 0) {
            self.white_long_castling = false;
            return;
        }

        if to_pos == TilePosition::new(7, 0) {
            self.white_short_castling = false;
            return;
        }

        if to_pos == TilePosition::new(0, 7) {
            self.black_long_castling = false;
            return;
        }

        if to_pos == TilePosition::new(7, 7) {
            self.black_short_castling = false;
            return;
        }
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
        Self {
            board: Board::default(),
            current_player: Player::White,

            en_passant_target: None,
            white_short_castling: true,
            white_long_castling: true,
            black_short_castling: true,
            black_long_castling: true,
        }
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