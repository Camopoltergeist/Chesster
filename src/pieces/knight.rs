use crate::{board::{bitboard::Bitboard, board::Board, tile_position::TilePosition}, piece::{Piece, PieceType}, player::Player};

use const_for::const_for;

pub struct Knight {
    player: Player,
    tile_position: TilePosition
}

impl Knight {
    pub const MOVEMENT_MASKS: [Bitboard; 64] = Self::generate_all_movement_masks();

    pub fn new(player: Player, tile_position: TilePosition) -> Self {
        Self {
            player,
            tile_position
        }
    }

    pub const fn generate_collision_mask(board: &Board, player: Player, tile_pos: TilePosition) -> Bitboard {
        Bitboard(
            Knight::MOVEMENT_MASKS[tile_pos.bit_offset() as usize].value()
                & !board.get_player_bitboard(player).value(),
        )
    }

    pub const fn get_movement_mask(tile_position: TilePosition) -> Bitboard {
        Self::MOVEMENT_MASKS[tile_position.bit_offset() as usize]
    }

    pub const fn generate_movement_mask(tile_position: TilePosition) -> Bitboard {
        Bitboard::generate_knight_mask(tile_position)
    }

    pub const fn generate_all_movement_masks() -> [Bitboard; 64] {
        let mut masks = [Bitboard(0); 64];

        const_for!(rank in 0..8 => {
            const_for!(column in 0..8 => {
                let tile_position = TilePosition::new(column, rank);

                masks[tile_position.bit_offset() as usize] = Knight::generate_movement_mask(tile_position);
            });
        });

        masks
    }

    pub const fn material_value() -> u32 {
        3
    }

    pub const fn phase_material_value() -> (i32, i32) {
        ( 426, 475)
    }    
}

impl Piece for Knight {
    fn piece_type(&self) -> PieceType {
        PieceType::Knight
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