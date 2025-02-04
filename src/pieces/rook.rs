use crate::{board::{bitboard::Bitboard, board::Board, move_collision::{get_cut_mask_horizontal, get_cut_mask_vertical}, tile_position::TilePosition}, piece::{Piece, PieceType}, player::Player};

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

    pub const fn generate_collision_mask(board: &Board, player: Player, tile_pos: TilePosition) -> Bitboard {
        let bit_offset = tile_pos.bit_offset();

        let mut valid_moves: u64 = Rook::MOVEMENT_MASKS[bit_offset as usize].value();

        let mut collision_mask: u64 = (board.white_pieces.value() | board.black_pieces.value()) & valid_moves;

        if collision_mask == 0 {
            return Bitboard(valid_moves);
        };

        let rank_mask: u64 = 0xFE;

        let n_collision = get_cut_mask_vertical(bit_offset, 8 - bit_offset / 8) & collision_mask;

        if n_collision != 0 {
            let n_offset = n_collision.trailing_zeros();
            if board
                .get_player_bitboard(player.opposite())
                .check_bit(n_offset)
            {
                valid_moves &= !get_cut_mask_vertical(n_offset + 8, 7 - (n_offset / 8));
            } else {
                valid_moves &= !get_cut_mask_vertical(n_offset, 8 - (n_offset / 8));
            }
            collision_mask &= !n_collision;
        }

        let w_collision = (rank_mask << bit_offset) & collision_mask;

        if w_collision != 0 {
            let w_offset = w_collision.trailing_zeros();
            if board
                .get_player_bitboard(player.opposite())
                .check_bit(w_offset)
            {
                valid_moves &= !(get_cut_mask_horizontal(w_offset + 1, 7 - w_offset % 8));
            } else {
                valid_moves &= !(get_cut_mask_horizontal(w_offset, 8 - w_offset % 8));
            }
            collision_mask &= !w_collision;
        }

        let s_collision = get_cut_mask_vertical(bit_offset % 8, bit_offset / 8) & collision_mask;

        if s_collision != 0 {
            let s_offset = 63 - s_collision.leading_zeros();
            if board
                .get_player_bitboard(player.opposite())
                .check_bit(s_offset)
            {
                valid_moves &= !(get_cut_mask_vertical(s_offset % 8, s_offset / 8));
            } else {
                valid_moves &= !(get_cut_mask_vertical(s_offset % 8, s_offset / 8 + 1));
            }
            collision_mask &= !s_collision;
        }

        if collision_mask != 0 {
            let e_offset = 63 - collision_mask.leading_zeros();
            if board
                .get_player_bitboard(player.opposite())
                .check_bit(e_offset)
            {
                valid_moves &= !(get_cut_mask_horizontal(e_offset - e_offset % 8, e_offset % 8));
            } else {
                valid_moves &= !(get_cut_mask_horizontal(e_offset - e_offset % 8, e_offset % 8 + 1));
            }
        }

        Bitboard(valid_moves)
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