use crate::piece::Piece;

use super::{bitboard::Bitboard, move_collision::{get_collision_mask, get_pawn_capture}, position::Position, tile_position::TilePosition};

///Returns a bitboard of every tile the opposite player can move to on their next turn
pub fn get_opposite_attack_mask(position: Position) -> Bitboard {
    let enemy_board: Bitboard = *position.board().get_player_bitboard(position.current_player().opposite().clone());

    let mut attack_mask: Bitboard = Bitboard(0);
    for bit_offset in 0..64 {
        if !enemy_board.check_bit(bit_offset) {
            continue;
        }

        let tile_pos = TilePosition::from_bit_offset(bit_offset);
        if let Some(player_piece) = position.get_piece(tile_pos) {
            attack_mask |= match player_piece.piece() {
                Piece::Pawn => Bitboard(get_pawn_capture(player_piece.player(), tile_pos)),
                _ => get_collision_mask(position.board().clone(), tile_pos),
            };
            }
    }

    attack_mask
}