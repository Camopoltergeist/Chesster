use crate::{board::{bitboard::Bitboard, move_mask::{BLACK_PAWN_MASKS, WHITE_PAWN_MASKS}, tile_position::TilePosition}, piece::{Piece, PieceType}, player::Player};

pub struct Pawn {
    player: Player,
    tile_position: TilePosition
}

impl Pawn {
    pub fn new(player: Player, tile_position: TilePosition) -> Self {
        Self {
            player,
            tile_position
        }
    }

    pub const fn get_movement_mask(tile_position: TilePosition, player: Player) -> Bitboard {
        match player {
            Player::White => WHITE_PAWN_MASKS[tile_position.bit_offset() as usize],
            Player::Black => BLACK_PAWN_MASKS[tile_position.bit_offset() as usize]
        }
    }

    pub const fn generate_movement_mask(tile_position: TilePosition, player: Player) -> Bitboard {
        match player {
            Player::White => Bitboard::get_white_pawn_mask(tile_position.column(), tile_position.rank()),
            Player::Black => Bitboard::get_black_pawn_mask(tile_position.column(), tile_position.rank())
        }
    }
}

impl Piece for Pawn {
    fn piece_type(&self) -> PieceType {
        PieceType::Pawn
    }

    fn player(&self) -> Player {
        self.player
    }

    fn tile_position(&self) -> TilePosition {
        self.tile_position
    }

    fn movement_mask(&self) -> Bitboard {
        Self::get_movement_mask(self.tile_position, self.player)
    }
}