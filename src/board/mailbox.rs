//! Module for `Mailbox` struct, which holds an array of PlayerPieces indexed by their bit offset.

use crate::player_piece::PlayerPiece;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mailbox {
    pub piece_array: [Option<PlayerPiece>; 64],
}

impl Mailbox {
    pub fn empty() -> Self {
        Self {
            piece_array: [None; 64],
        }
    }

    pub fn set_piece(&mut self, piece: PlayerPiece, bit_offset: u32) {
        debug_assert!(bit_offset < 64);
        self.piece_array[bit_offset as usize] = Some(piece)
    }

    pub fn remove_piece(&mut self, bit_offset: u32) {
        debug_assert!(bit_offset < 64);
        self.piece_array[bit_offset as usize] = None
    }

    pub fn get_piece(&self, bit_offset: u32) -> Option<PlayerPiece> {
        debug_assert!(bit_offset < 64);
        self.piece_array[bit_offset as usize]
    }

    pub fn move_piece(&mut self, from_offset: u32, to_offset: u32) {
        debug_assert!(from_offset < 64);
        debug_assert!(to_offset < 64);
        self.piece_array[to_offset as usize] = self.piece_array[from_offset as usize];
        self.remove_piece(from_offset)
    }
    
}

