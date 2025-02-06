use crate::player::Player;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    Ongoing,
    Checkmate(Player),
    Stalemate
}

impl GameState {
    pub fn has_ended(&self) -> bool {
        !(*self == Self::Ongoing)
    }
}