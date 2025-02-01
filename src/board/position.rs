use crate::{player::Player, player_piece::PlayerPiece};

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

        collision_mask.check_bit(moove.to().bit_offset())
    }

    pub fn make_move(&mut self, moove: Move) -> Result<(), ()> {
        if !self.is_legal_move(&moove) {
            return Err(());
        };

        let piece = self.board.get_piece(moove.from()).unwrap();

        if piece.player() != self.current_player {
            return Err(());
        };

        self.board.move_piece(self.current_player, piece.piece(), moove.from().bit_offset(), moove.to().bit_offset());

        self.current_player = self.current_player.opposite();

        Ok(())
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
