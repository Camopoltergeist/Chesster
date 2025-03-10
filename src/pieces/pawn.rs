use crate::{board::{bitboard::Bitboard, board::Board, move_collision::get_pawn_capture, tile_position::TilePosition}, piece::{Piece, PieceType}, player::Player};

use const_for::const_for;

pub struct Pawn {
    player: Player,
    tile_position: TilePosition
}

impl Pawn {
    pub const WHITE_MOVEMENT_MASKS: [Bitboard; 64] = Self::generate_all_movement_masks(Player::White);
    pub const BLACK_MOVEMENT_MASKS: [Bitboard; 64] = Self::generate_all_movement_masks(Player::Black);

    pub fn new(player: Player, tile_position: TilePosition) -> Self {
        Self {
            player,
            tile_position
        }
    }

    pub const fn generate_collision_mask(board: &Board, player: Player, tile_pos: TilePosition) -> Bitboard {
        let collision_mask = board.white_pieces.value() | board.black_pieces.value();

        let valid_moves = match player {
            Player::White => {
                if (1 << tile_pos.bit_offset() + 8) & collision_mask != 0 {
                    0
                } else {
                    Pawn::WHITE_MOVEMENT_MASKS[tile_pos.bit_offset() as usize].value() & !collision_mask
                }
            }
            Player::Black => {
                if (1 << tile_pos.bit_offset() - 8) & collision_mask != 0 {
                    0
                } else {
                    Pawn::BLACK_MOVEMENT_MASKS[tile_pos.bit_offset() as usize].value() & !collision_mask
                }
            }
        };

        let capture_moves =
            get_pawn_capture(player, tile_pos) & board.get_player_bitboard(player.opposite()).value();
        Bitboard(valid_moves | capture_moves)
    }

    pub const fn get_movement_mask(tile_position: TilePosition, player: Player) -> Bitboard {
        match player {
            Player::White => Self::WHITE_MOVEMENT_MASKS[tile_position.bit_offset() as usize],
            Player::Black => Self::BLACK_MOVEMENT_MASKS[tile_position.bit_offset() as usize]
        }
    }

    pub const fn generate_movement_mask(tile_position: TilePosition, player: Player) -> Bitboard {
        match player {
            Player::White => Bitboard::get_white_pawn_mask(tile_position.column(), tile_position.rank()),
            Player::Black => Bitboard::get_black_pawn_mask(tile_position.column(), tile_position.rank())
        }
    }

    pub const fn generate_all_movement_masks(player: Player) -> [Bitboard; 64] {
        let mut masks = [Bitboard(0); 64];

        const_for!(rank in 0..8 => {
            const_for!(column in 0..8 => {
                let tile_pos = TilePosition::new(column, rank);

                masks[tile_pos.bit_offset() as usize] = Pawn::generate_movement_mask(tile_pos, player);
            });
        });

        masks
    }

    pub const fn material_value() -> u32 {
        1
    }

    pub const fn phase_material_value() -> (i32, i32) {
        (  82, 144)
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