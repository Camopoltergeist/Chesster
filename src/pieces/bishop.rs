use crate::{
    board::{
        bitboard::Bitboard,
        board::Board,
        move_collision::{ne_collision_cut_mask, nw_collision_cut_mask, se_collision_cut_mask, sw_collision_cut_mask},
        tile_position::TilePosition,
    },
    piece::{Piece, PieceType},
    player::Player,
};

use const_for::const_for;

pub struct Bishop {
    player: Player,
    tile_position: TilePosition,
}

impl Bishop {
    pub const MOVEMENT_MASKS: [Bitboard; 64] = Self::generate_all_movement_masks();

    pub fn new(player: Player, tile_position: TilePosition) -> Self {
        Self {
            player,
            tile_position,
        }
    }

    pub fn generate_collision_mask(
        board: &Board,
        player: Player,
        tile_pos: TilePosition,
    ) -> Bitboard {
        let mut valid_moves = Bishop::MOVEMENT_MASKS[tile_pos.bit_offset() as usize];
        let mut collision_mask =
            (board.white_pieces | board.black_pieces) & valid_moves;

        if collision_mask == 0 {
            return valid_moves;
        }

        let column = tile_pos.column();
        let rank = tile_pos.rank();
        let offset = tile_pos.bit_offset();

        let ne_collision =
            Bitboard::get_diagonal_mask_asc(0, 0) << offset as u64 & collision_mask;

        if ne_collision != 0 {
            valid_moves &= !ne_collision_cut_mask(board, ne_collision, player);
            collision_mask &= !ne_collision;
        }

        let nw_collision =
            (Bitboard::get_diagonal_mask_des(7, 0) >> 7) << offset as u64 & collision_mask;
        if nw_collision != 0 {
            valid_moves &= !nw_collision_cut_mask(board, nw_collision, player);
            collision_mask &= !nw_collision;
        }

        let sw_collision = Bitboard::get_diagonal_mask_asc(column, rank) & collision_mask;
        if sw_collision != 0 {
            valid_moves &= !sw_collision_cut_mask(board, sw_collision, player);
            collision_mask &= !sw_collision;
        }

        if collision_mask != 0 {
            valid_moves &= !se_collision_cut_mask(board, collision_mask, player);
        }

        Bitboard::print_bitboard(&valid_moves);
        valid_moves
    }

    pub const fn get_movement_mask(tile_position: TilePosition) -> Bitboard {
        Self::MOVEMENT_MASKS[tile_position.bit_offset() as usize]
    }

    pub const fn generate_movement_mask(tile_position: TilePosition) -> Bitboard {
        let rank_mask =
            Bitboard::get_diagonal_mask_asc(tile_position.column(), tile_position.rank());
        let column_mask =
            Bitboard::get_diagonal_mask_des(tile_position.column(), tile_position.rank());

        Bitboard(rank_mask.0 ^ column_mask.0)
    }

    pub const fn generate_all_movement_masks() -> [Bitboard; 64] {
        let mut masks = [Bitboard(0); 64];

        const_for!(rank in 0..8 => {
            const_for!(column in 0..8 => {
                let tile_pos = TilePosition::new(column, rank);

                let mask = Bishop::generate_movement_mask(tile_pos);

                masks[tile_pos.bit_offset() as usize] = mask;
            })
        });

        masks
    }

    pub const fn material_value() -> u32 {
        3
    }
}

impl Piece for Bishop {
    fn piece_type(&self) -> PieceType {
        PieceType::Bishop
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
