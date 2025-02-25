use crate::piece::PieceType;

use super::{
    bitboard::Bitboard,
    move_collision::{get_collision_mask, get_pawn_capture},
    position::Position,
    tile_position::TilePosition,
};

///Returns a bitboard of every tile the opposite player can move to on their next turn
pub fn get_opposite_attack_mask(position: Position) -> Bitboard {
    let mut opponent_pieces_mask: Bitboard = *position
        .board()
        .get_player_bitboard(position.current_player().opposite().clone());

    let mut attack_mask: Bitboard = Bitboard(0);

    while !opponent_pieces_mask.is_empty() {
        let bit_offset = opponent_pieces_mask.0.trailing_zeros();

        let tile_pos = TilePosition::from_bit_offset(bit_offset);
        if let Some(player_piece) = position.get_piece(tile_pos) {
            attack_mask |= match player_piece.piece() {
                PieceType::Pawn => Bitboard(get_pawn_capture(player_piece.player(), tile_pos)),
                _ => get_collision_mask(position.board().clone(), tile_pos),
            };
        }

        opponent_pieces_mask.unset_bit(bit_offset);
    }

    attack_mask
}
