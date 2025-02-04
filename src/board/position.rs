use crate::{piece::PieceType, player::Player, player_piece::PlayerPiece};

use super::{board::Board, moove::Move, move_collision::get_collision_mask, tile_position::TilePosition};

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

    pub fn get_all_legal_moves(&self) -> Vec<Move> {
        let piece_mask = self.board.get_player_bitboard(self.current_player);

        let mut legal_moves = Vec::new();

        for bit_offset in 0..64 {
            if !piece_mask.check_bit(bit_offset) {
                continue;
            }

            let tile_pos = TilePosition::from_bit_offset(bit_offset);

            legal_moves.append(&mut self.get_legal_moves(tile_pos));
        };

        return legal_moves;
    }

    pub fn get_legal_moves(&self, tile_pos: TilePosition) -> Vec<Move> {
        let mut legal_moves = Vec::new();

        if let Some(_) = self.board.get_piece(tile_pos) {
            let moves_bitboard = get_collision_mask(self.board.clone(), tile_pos);

            if moves_bitboard.is_empty() {
                return legal_moves;
            }

            for bit_offset in 0..64 {
                if moves_bitboard.check_bit(bit_offset) {
                    legal_moves.push(Move::new(tile_pos, TilePosition::from_bit_offset(bit_offset)));
                }
            }
        }

        return legal_moves;
    }

    pub fn get_piece(&self, tile_pos: TilePosition) -> Option<PlayerPiece> {
        self.board.get_piece(tile_pos)
    } 

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn is_legal_move(&self, moove: &Move) -> bool {
        let collision_mask = get_collision_mask(self.board.clone(), moove.from());
        if !collision_mask.check_bit(moove.to().bit_offset()) {
            return false;
        };

        let piece = self.board.get_piece(moove.from()).unwrap();

        if piece.player() != self.current_player {
            return false;
        };

        return true;
    }

    pub fn make_move(&mut self, moove: Move) -> Result<(), ()> {
        if !self.is_legal_move(&moove) {
            return Err(());
        };

        self.board.move_piece(moove);

        self.current_player = self.current_player.opposite();

        Ok(())
    }

    pub fn current_player(&self) -> Player {
        self.current_player
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