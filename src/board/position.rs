use crate::{board::moove::CastleSide, piece::PieceType, player::Player, player_piece::PlayerPiece};

use super::{board::Board, moove::{BasicMove, CastlingMove, Move}, move_collision::get_collision_mask, tile_position::TilePosition};

#[derive(Clone)]
pub struct Position {
    board: Board,
    current_player: Player,

    en_passant: bool,
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

    pub fn get_all_legal_moves(&self) -> Vec<Move> {
        let piece_mask = self.board.get_player_bitboard(self.current_player);

        let mut legal_moves = Vec::new();

        for bit_offset in 0..64 {
            if !piece_mask.check_bit(bit_offset) {
                continue;
            }

            let tile_pos = TilePosition::from_bit_offset(bit_offset);

            legal_moves.append(&mut self.get_basic_moves_for_tile_position(tile_pos));
        };

        return legal_moves;
    }

    pub fn get_basic_moves_for_tile_position(&self, tile_pos: TilePosition) -> Vec<Move> {
        let mut legal_moves = Vec::new();

        if let Some(piece) = self.board.get_piece(tile_pos) {
            let moves_bitboard = get_collision_mask(self.board.clone(), tile_pos);

            if moves_bitboard.is_empty() {
                return legal_moves;
            }

            for bit_offset in 0..64 {
                if moves_bitboard.check_bit(bit_offset) {
                    legal_moves.push(BasicMove::new(tile_pos, TilePosition::from_bit_offset(bit_offset)).into());
                }
            }

            if piece.piece() == PieceType::King {
                let castling_moves = self.get_legal_castling_moves();
                legal_moves.extend(castling_moves);
            }
        }

        return legal_moves;
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
            _ => unimplemented!()
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

    pub fn make_move(&mut self, moove: Move) -> Result<(), ()> {
        if !self.is_legal_move(&moove) {
            return Err(());
        };

        self.change_castling_availability_if_needed(&moove);

        match moove {
            Move::Basic(basic_move) => self.board.move_piece_basic(basic_move),
            Move::Castling(castling_move) => {
                
                self.board.move_piece_castling(castling_move);
            }
            _ => unimplemented!()
        }

        self.current_player = self.current_player.opposite();

        Ok(())
    }

    fn set_castling_availability(&mut self, player: Player, side: CastleSide, value: bool) {
        match (player, side) {
            (Player::White, CastleSide::KingSide) => self.white_short_castling = value,
            (Player::White, CastleSide::QueenSide) => self.white_long_castling = value,
            (Player::Black, CastleSide::KingSide) => self.black_short_castling = value,
            (Player::Black, CastleSide::QueenSide) => self.black_long_castling = value
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

            en_passant: false,
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