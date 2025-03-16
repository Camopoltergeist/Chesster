//! Board representation and manipulation module.
//! 
//! This module defines the `Board` struct, which represents the chessboard state and provides methods for manipulating and querying the board.

use super::{
    bitboard::Bitboard, mailbox::Mailbox, moove::{BasicMove, CastleSide, CastlingMove, EnPassantMove, PromotingMove}, move_collision::{get_collision_mask, get_pawn_capture}, tile_position::TilePosition
};
use crate::{
    piece::PieceType,
    pieces::{bishop::Bishop, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook},
    player::Player,
    player_piece::PlayerPiece,
};

/// Represents the chessboard state using bitboards and a mailbox array.
#[derive(Clone, PartialEq, Eq)]
pub struct Board {
    pub white_pieces: Bitboard,
    pub black_pieces: Bitboard,
    pub pawns: Bitboard,
    pub rooks: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub queens: Bitboard,
    pub kings: Bitboard,

    mailbox: Mailbox,
}

/// Provides the default starting position of the board.
impl Default for Board {
    fn default() -> Self {
        Self {
            white_pieces: Bitboard(
                0b_0000000_0000000_0000000_0000000_0000000_00000000_11111111_11111111,
            ),
            black_pieces: Bitboard(
                0b_11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
            ),
            pawns: Bitboard(
                0b_00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000,
            ),
            rooks: Bitboard(
                0b_10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            ),
            knights: Bitboard(
                0b_01000010_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            ),
            bishops: Bitboard(
                0b_00100100_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            ),
            queens: Bitboard(
                0b_00001000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            ),
            kings: Bitboard(
                0b_00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            ),
            mailbox: Mailbox {
                piece_array: [
                    Some(PlayerPiece::new(Player::White, PieceType::Rook)),
                    Some(PlayerPiece::new(Player::White, PieceType::Knight)),
                    Some(PlayerPiece::new(Player::White, PieceType::Bishop)),
                    Some(PlayerPiece::new(Player::White, PieceType::Queen)),
                    Some(PlayerPiece::new(Player::White, PieceType::King)),
                    Some(PlayerPiece::new(Player::White, PieceType::Bishop)),
                    Some(PlayerPiece::new(Player::White, PieceType::Knight)),
                    Some(PlayerPiece::new(Player::White, PieceType::Rook)),
                    Some(PlayerPiece::new(Player::White, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::White, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::White, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::White, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::White, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::White, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::White, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::White, PieceType::Pawn)),
                    None, None, None, None, None, None, None, None,
                    None, None, None, None, None, None, None, None,
                    None, None, None, None, None, None, None, None,
                    None, None, None, None, None, None, None, None,
                    Some(PlayerPiece::new(Player::Black, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Pawn)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Rook)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Knight)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Bishop)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Queen)),
                    Some(PlayerPiece::new(Player::Black, PieceType::King)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Bishop)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Knight)),
                    Some(PlayerPiece::new(Player::Black, PieceType::Rook)),
                ]
            }
        }
    }
}

impl Board {
    /// Creates an empty board with no pieces.
    pub fn empty() -> Self {
        Self {
            white_pieces: Bitboard(0),
            black_pieces: Bitboard(0),
            pawns: Bitboard(0),
            rooks: Bitboard(0),
            kings: Bitboard(0),
            knights: Bitboard(0),
            bishops: Bitboard(0),
            queens: Bitboard(0),
            mailbox: Mailbox::empty(),
        }
    }

    /// Checks for overlapping pieces between two bitboards.
    pub fn check_overlaps(a: Bitboard, b: Bitboard) -> bool {
        a & b != 0
    }

    /// Validates that no two different piece types occupy the same tile.
    pub fn validate(&self) -> bool {
        !(Board::check_overlaps(self.pawns, self.rooks)
            || Board::check_overlaps(self.pawns, self.knights)
            || Board::check_overlaps(self.pawns, self.bishops)
            || Board::check_overlaps(self.pawns, self.queens)
            || Board::check_overlaps(self.pawns, self.kings)
            || Board::check_overlaps(self.rooks, self.knights)
            || Board::check_overlaps(self.rooks, self.bishops)
            || Board::check_overlaps(self.rooks, self.queens)
            || Board::check_overlaps(self.rooks, self.kings)
            || Board::check_overlaps(self.knights, self.bishops)
            || Board::check_overlaps(self.knights, self.queens)
            || Board::check_overlaps(self.knights, self.kings)
            || Board::check_overlaps(self.bishops, self.queens)
            || Board::check_overlaps(self.bishops, self.kings)
            || Board::check_overlaps(self.queens, self.kings))
    }

    /// Returns a reference to the mailbox.
    pub fn mailbox(&self) -> &Mailbox {
        &self.mailbox
    }

    /// Moves a piece
    pub fn move_piece_basic(&mut self, basic_move: BasicMove) {
        let piece = self
            .get_piece(basic_move.from_position())
            .expect("no piece at move's \"from\" tile");

        self.set_piece(piece, basic_move.to_position());
        self.remove_piece(basic_move.from_position());
    }

    pub fn move_piece_castling(&mut self, castling_move: CastlingMove) {
        self.move_piece_basic(castling_move.king_basic_move());
        self.move_piece_basic(castling_move.rook_basic_move());
    }

    pub fn move_piece_en_passant(&mut self, en_passant_move: EnPassantMove) {
        self.remove_piece(en_passant_move.captured_tile());
        self.move_piece_basic(en_passant_move.into());
    }

    pub fn move_piece_promoting(&mut self, promoting_move: PromotingMove) {
        self.move_piece_basic(promoting_move.clone().into());
        self.set_piece(
            promoting_move.promotion_piece(),
            promoting_move.to_position(),
        );
    }

    /// Returns PieceType (or none) from the mailbox.
    pub fn get_piece_from_offset(&self, bit_offset: u32) -> Option<PlayerPiece> {
        self.mailbox.get_piece(bit_offset)
    }

        /// Returns PieceType (or none) from the bitboard.
    pub fn get_piece_from_offset_bitboard(&self, bit_offset: u32) -> Option<PlayerPiece> {
        let player = if Bitboard::check_bit(&self.white_pieces, bit_offset) {
            Player::White
        } else if Bitboard::check_bit(&self.black_pieces, bit_offset) {
            Player::Black
        } else {
            return None;
        };

        let piece = match () {
            _ if self.pawns.check_bit(bit_offset) => PieceType::Pawn,
            _ if self.rooks.check_bit(bit_offset) => PieceType::Rook,
            _ if self.knights.check_bit(bit_offset) => PieceType::Knight,
            _ if self.bishops.check_bit(bit_offset) => PieceType::Bishop,
            _ if self.queens.check_bit(bit_offset) => PieceType::Queen,
            _ if self.kings.check_bit(bit_offset) => PieceType::King,
            _ => return None,
        };

        Some(PlayerPiece::new(player, piece))
    }

    pub fn check_for_pawn(&self, tile_pos: TilePosition) -> bool {
        self.pawns.check_bit(tile_pos.bit_offset())
    }

    pub const fn get_player_bitboard(&self, player: Player) -> &Bitboard {
        match player {
            Player::White => &self.white_pieces,
            Player::Black => &self.black_pieces,
        }
    }

    pub const fn get_player_bitboard_mut(&mut self, player: Player) -> &mut Bitboard {
        match player {
            Player::White => &mut self.white_pieces,
            Player::Black => &mut self.black_pieces,
        }
    }

    pub const fn get_piece_bitboard(&self, piece: PieceType) -> &Bitboard {
        match piece {
            PieceType::Pawn => &self.pawns,
            PieceType::Rook => &self.rooks,
            PieceType::Knight => &self.knights,
            PieceType::Bishop => &self.bishops,
            PieceType::Queen => &self.kings,
            PieceType::King => &self.kings,
        }
    }

    pub const fn get_piece_bitboard_mut(&mut self, piece: PieceType) -> &mut Bitboard {
        match piece {
            PieceType::Pawn => &mut self.pawns,
            PieceType::Rook => &mut self.rooks,
            PieceType::Knight => &mut self.knights,
            PieceType::Bishop => &mut self.bishops,
            PieceType::Queen => &mut self.queens,
            PieceType::King => &mut self.kings,
        }
    }

    pub fn remove_piece_from_offset(&mut self, bit_offset: u32) {
        self.white_pieces.unset_bit(bit_offset);
        self.black_pieces.unset_bit(bit_offset);

        self.pawns.unset_bit(bit_offset);
        self.rooks.unset_bit(bit_offset);
        self.knights.unset_bit(bit_offset);
        self.bishops.unset_bit(bit_offset);
        self.queens.unset_bit(bit_offset);
        self.kings.unset_bit(bit_offset);

        self.mailbox.remove_piece(bit_offset);
    }

    pub fn set_piece_to_offset(&mut self, piece: PlayerPiece, bit_offset: u32) {
        self.remove_piece_from_offset(bit_offset);

        self.get_player_bitboard_mut(piece.player())
            .set_bit(bit_offset);
        self.get_piece_bitboard_mut(piece.piece())
            .set_bit(bit_offset);

        self.mailbox.set_piece(piece, bit_offset);
    }

    /// Returns a Bitboard mask of all pieces on the board.
    pub const fn get_all_pieces_mask(&self) -> Bitboard {
        Bitboard(self.black_pieces.value() | self.white_pieces.value())
    }

    /// Returns the player color on a specific TilePosition. (None = The square is empty.)
    pub fn get_player_at(&self, tile_pos: TilePosition) -> Option<Player> {
        let bit_offset = tile_pos.bit_offset();

        if self.white_pieces.check_bit(bit_offset) {
            return Some(Player::White);
        }

        if self.black_pieces.check_bit(bit_offset) {
            return Some(Player::Black);
        }

        return None;
    }

    /// Returns the PlayerPiece on a specifc TilePosition. (None = the square is empty.)
    pub fn get_piece(&self, tile_pos: TilePosition) -> Option<PlayerPiece> {
        return self.get_piece_from_offset(tile_pos.bit_offset());
    }

    pub fn remove_piece(&mut self, tile_pos: TilePosition) {
        self.remove_piece_from_offset(tile_pos.bit_offset());
    }

    pub fn set_piece(&mut self, piece: PlayerPiece, tile_pos: TilePosition) {
        self.set_piece_to_offset(piece, tile_pos.bit_offset());
    }

    /// A debugging function using string format for tile selection (a1, b2 etc.)
    pub fn get_piece_debug(&self, tile_str: &str) -> Option<PlayerPiece> {
        let tile_pos = TilePosition::from_tile_str(tile_str).expect("invalid tile str passed");

        self.get_piece(tile_pos)
    }

    /// Checks if castling is possible for player and side of castling, returns true or false
    pub fn is_castling_possible(&self, player: Player, side: CastleSide) -> bool {
        let castling_block_mask = Bitboard::generate_castling_block_mask(player, side.clone());
        let castling_threat_mask = Bitboard::generate_castling_threat_mask(player, side);

        let is_blocking = self.get_all_pieces_mask() & castling_block_mask;
        let is_attacked = self.get_attack_mask(player.opposite()) & castling_threat_mask;

        return is_blocking.is_empty() && is_attacked.is_empty();
    }

    /// Returns a player's all possible tiles they can move to as a Bitboard mask
    pub fn get_attack_mask(&self, player: Player) -> Bitboard {
        let mut player_board = self.get_player_bitboard(player).clone();

        let mut attack_mask: Bitboard = Bitboard(0);

        while !player_board.is_empty() {
            let bit_offset = player_board.0.trailing_zeros();

            let tile_pos = TilePosition::from_bit_offset(bit_offset);

            if let Some(player_piece) = self.get_piece_from_offset(bit_offset) {
                attack_mask |= match player_piece.piece() {
                    PieceType::Pawn => Bitboard(get_pawn_capture(player_piece.player(), tile_pos)),
                    _ => get_collision_mask(self.clone(), tile_pos),
                };
            }

            player_board.unset_bit(bit_offset);
        }

        attack_mask
    }

    pub fn get_material_for_player(&self, player: Player) -> u32 {
        let player_bitboard = *self.get_player_bitboard(player);

        let mut material = 0;
        material += (self.pawns & player_bitboard).0.count_ones() * Pawn::material_value();
        material += (self.rooks & player_bitboard).0.count_ones() * Rook::material_value();
        material += (self.bishops & player_bitboard).0.count_ones() * Bishop::material_value();
        material += (self.knights & player_bitboard).0.count_ones() * Knight::material_value();
        material += (self.queens & player_bitboard).0.count_ones() * Queen::material_value();

        material
    }

    pub fn get_phase_material_for_player(&self, player: Player, game_phase: (i32, i32)) -> u32 {
        let player_bitboard = *self.get_player_bitboard(player);
        let mut material = 0;

        let pawns = (self.pawns & player_bitboard).0.count_ones();
        let pawn_phase_value = Pawn::phase_material_value().0 * game_phase.0 + Pawn::phase_material_value().1 + game_phase.1;
        material += pawns * pawn_phase_value as u32 / 100;

        let rooks = (self.rooks & player_bitboard).0.count_ones();
        let rook_phase_value = Rook::phase_material_value().0 * game_phase.0 + Rook::phase_material_value().1 + game_phase.1;
        material += rooks * rook_phase_value as u32 / 100;

        let bishops = (self.bishops & player_bitboard).0.count_ones();
        let bishop_phase_value = Bishop::phase_material_value().0 * game_phase.0 + Bishop::phase_material_value().1 + game_phase.1;
        material += bishops * bishop_phase_value as u32 / 100;


        let knights = (self.knights & player_bitboard).0.count_ones();
        let knight_phase_value = Knight::phase_material_value().0 * game_phase.0 + Knight::phase_material_value().1 + game_phase.1;
        material += knights * knight_phase_value as u32 / 100;

        let queens = (self.queens & player_bitboard).0.count_ones();
        let queen_phase_value = Queen::phase_material_value().0 * game_phase.0 + Queen::phase_material_value().1 + game_phase.1;
        material += queens * queen_phase_value as u32 / 100;

        material as u32
    }    
}
