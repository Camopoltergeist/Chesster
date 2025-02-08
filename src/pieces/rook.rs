use crate::{
    board::{
        bitboard::Bitboard,
        board::Board,
        move_collision::{
            e_collision_cut_mask, n_collision_cut_mask, s_collision_cut_mask, w_collision_cut_mask,
        },
        tile_position::TilePosition,
    },
    piece::{Piece, PieceType},
    player::Player,
};

use const_for::const_for;

pub struct Rook {
    player: Player,
    tile_position: TilePosition,
}

impl Rook {
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
        let mut valid_moves = Rook::MOVEMENT_MASKS[tile_pos.bit_offset() as usize];
        let mut collision_mask = (board.white_pieces | board.black_pieces) & valid_moves;

        if collision_mask == 0 {
            return valid_moves;
        };

        let column = tile_pos.column();
        let rank = tile_pos.rank();

        let n_collision =
            (Bitboard::generate_column_mask(column) << (rank as u64) * 8) & collision_mask;

        if n_collision != 0 {
            valid_moves &= !n_collision_cut_mask(board, n_collision, player);
            collision_mask &= !n_collision;
        }

        let e_collision = (Bitboard::generate_rank_mask(rank) << column as u64) & collision_mask;

        if e_collision != 0 {
            valid_moves &= !e_collision_cut_mask(board, e_collision, player);
            collision_mask &= !e_collision;
        }

        let s_collision = Bitboard::generate_column_mask(column) & collision_mask;

        if s_collision != 0 {
            valid_moves &= !s_collision_cut_mask(board, s_collision, player);
            collision_mask &= !s_collision;
        }

        if collision_mask != 0 {
            valid_moves &= !w_collision_cut_mask(board, collision_mask, player);
        }

        Bitboard::print_bitboard(&valid_moves);
        valid_moves
    }

    pub const fn get_movement_mask(tile_pos: TilePosition) -> Bitboard {
        Self::MOVEMENT_MASKS[tile_pos.bit_offset() as usize]
    }

    pub const fn generate_movement_mask(tile_pos: TilePosition) -> Bitboard {
        let rank_mask = Bitboard::generate_rank_mask(tile_pos.rank());
        let column_mask = Bitboard::generate_column_mask(tile_pos.column());

        return Bitboard(rank_mask.0 ^ column_mask.0);
    }

    pub const fn generate_all_movement_masks() -> [Bitboard; 64] {
        let mut masks = [Bitboard(0); 64];

        const_for!(rank in 0..8 => {
            const_for!(column in 0..8 => {
                let tile_pos = TilePosition::new(column, rank);
                let mask = Rook::generate_movement_mask(tile_pos);

                masks[tile_pos.bit_offset() as usize] = mask;
            })
        });

        masks
    }

    pub const fn material_value() -> u32 {
        5
    }
}

impl Piece for Rook {
    fn piece_type(&self) -> PieceType {
        PieceType::Rook
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
