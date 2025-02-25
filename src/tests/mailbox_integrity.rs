use crate::board::position::Position;

#[test]
fn mailbox_starting_position_identical_to_bitboard() {
    let position = Position::default();
    
    for bit_offset in 0..64 {
        let mailbox_piece = position.board().get_piece_from_offset(bit_offset);
        let bitboard_piece = position.board().get_piece_from_offset_bitboard(bit_offset);

        assert_eq!(mailbox_piece, bitboard_piece);
    }
}