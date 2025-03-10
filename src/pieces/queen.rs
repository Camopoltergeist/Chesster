use crate::{board::{bitboard::Bitboard, board::Board, tile_position::TilePosition}, piece::{Piece, PieceType}, player::Player};

use super::{bishop::Bishop, rook::Rook};

use const_for::const_for;

pub struct Queen {
    player: Player,
    tile_position: TilePosition
}

impl Queen {
    pub const MOVEMENT_MASKS: [Bitboard; 64] = Self::generate_all_movement_masks();

    pub fn new(player: Player, tile_position: TilePosition) -> Self {
        Self {
            player,
            tile_position
        }
    }

    pub fn generate_collision_mask(board: &Board, player: Player, tile_pos: TilePosition) -> Bitboard {
        Rook::generate_collision_mask(&board, player, tile_pos) | Bishop::generate_collision_mask(&board, player, tile_pos)
    }

    pub const fn get_movement_mask(tile_position: TilePosition) -> Bitboard {
        Self::MOVEMENT_MASKS[tile_position.bit_offset() as usize]
    }

    pub const fn generate_movement_mask(tile_position: TilePosition) -> Bitboard {
        let index = tile_position.bit_offset() as usize;

        let rook_mask = Rook::MOVEMENT_MASKS[index];
        let bishop_mask = Bishop::MOVEMENT_MASKS[index];

        let combined = rook_mask.0 | bishop_mask.0;

        Bitboard(combined)
    }

    pub const fn generate_all_movement_masks() -> [Bitboard; 64] {
        let mut masks = [Bitboard(0); 64];

        const_for!(bit_offset in 0..masks.len() => {
            masks[bit_offset] = Self::generate_movement_mask(TilePosition::from_bit_offset(bit_offset as u32));
        });

        masks
    }

    pub const fn material_value() -> u32 {
        9
    }

    pub const fn phase_material_value() -> (i32, i32) {
        (1292,1623)
    }    
}

impl Piece for Queen {
    fn piece_type(&self) -> PieceType {
        PieceType::Queen
    }

    fn player(&self) -> Player {
        self.player
    }

    fn tile_position(&self) -> TilePosition {
        self.tile_position
    }

    fn movement_mask(&self) -> Bitboard {
        Self::get_movement_mask(self.tile_position)
    }
}