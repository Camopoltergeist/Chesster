use crate::{board::{bitboard::Bitboard, board::Board, move_collision::{get_cut_mask_asc, get_cut_mask_des}, tile_position::TilePosition}, piece::{Piece, PieceType}, player::Player};

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
            tile_position
        }
    }

    pub fn generate_collision_mask(board: &Board, player: Player, tile_pos: TilePosition) -> Bitboard {
        let mut valid_moves: u64 = Bishop::MOVEMENT_MASKS[tile_pos.bit_offset() as usize].value();
        let column = tile_pos.column();
        let rank = tile_pos.rank();
        let offset = tile_pos.bit_offset();

        let mut collision_mask: u64 = (board.white_pieces.value() | board.black_pieces.value()) & valid_moves;

        if collision_mask == 0 {
            return Bitboard(valid_moves);
        }

        let ne_collision: u64 = get_cut_mask_asc(offset, u32::min(8 - column, 8 - rank)) & collision_mask;

        if ne_collision != 0 {
            let ne_offset = TilePosition::from_bit_offset(ne_collision.trailing_zeros());
            if board
                .get_player_bitboard(player.opposite())
                .check_bit(ne_offset.bit_offset())
            {
                valid_moves &=
                    !(get_cut_mask_asc(ne_offset.bit_offset(), u32::min(7 - column, 7 - rank)) << 9);
            } else {
                valid_moves &= !get_cut_mask_asc(ne_offset.bit_offset(), u32::min(8 - column, 8 - rank));
            }
            collision_mask &= !ne_collision;
        }

        let nw_collision: u64 = get_cut_mask_des(offset, u32::min(column + 1, 8 - rank)) & collision_mask;
        if nw_collision != 0 {
            let nw_offset = TilePosition::from_bit_offset(nw_collision.trailing_zeros());
            if board
                .get_player_bitboard(player.opposite())
                .check_bit(nw_offset.bit_offset())
            {
                let distance = u32::min(nw_offset.column(), 8 - nw_offset.rank());
                valid_moves &= !(get_cut_mask_des(nw_offset.bit_offset(), distance) << 7);
            } else {
                let distance = u32::min(nw_offset.column(), 8 - nw_offset.rank());
                valid_moves &= !get_cut_mask_des(nw_offset.bit_offset(), distance + 1);
            }
            collision_mask &= !nw_collision;
        }

        let sw_collision = (Bitboard::get_diagonal_mask_asc(column, rank).value()) & collision_mask;
        if sw_collision != 0 {
            let sw_offset = TilePosition::from_bit_offset(63 - sw_collision.leading_zeros());
            if board
                .get_player_bitboard(player.opposite())
                .check_bit(sw_offset.bit_offset())
            {
                let distance = u32::min(sw_offset.column(), sw_offset.rank());
                valid_moves &= !get_cut_mask_asc(sw_offset.bit_offset() - distance * 9, distance);
            } else {
                let distance = u32::min(sw_offset.column(), sw_offset.rank());
                valid_moves &= !get_cut_mask_asc(sw_offset.bit_offset() - distance * 9, distance + 1);
            }
            collision_mask &= !sw_collision;
        }

        if collision_mask != 0 {
            let se_offset = TilePosition::from_bit_offset(63 - collision_mask.leading_zeros());
            if board
                .get_player_bitboard(player.opposite())
                .check_bit(se_offset.bit_offset())
            {
                let distance = u32::min(8 - se_offset.column(), se_offset.rank());
                let attack_offset;
                if se_offset.bit_offset() > 7 {
                    attack_offset = se_offset.bit_offset() - 7;
                } else {
                    attack_offset = se_offset.bit_offset()
                };

                const_for!(i in 0..distance => {
                    valid_moves &= !(1 << attack_offset - 7 * i);
                });

            } else {
                let distance = u32::min(8 - se_offset.column(), se_offset.rank() + 1);
                for i in 0..distance {
                    valid_moves &= !(1 << se_offset.bit_offset() - 7 * i);
                }
            }
        }

        Bitboard(valid_moves)
    }

    pub const fn get_movement_mask(tile_position: TilePosition) -> Bitboard {
        Self::MOVEMENT_MASKS[tile_position.bit_offset() as usize]
    }

    pub const fn generate_movement_mask(tile_position: TilePosition) -> Bitboard {
        let rank_mask = Bitboard::get_diagonal_mask_asc(tile_position.column(), tile_position.rank());
        let column_mask = Bitboard::get_diagonal_mask_des(tile_position.column(), tile_position.rank());

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