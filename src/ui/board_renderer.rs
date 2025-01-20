use crate::player::Player;

pub struct BoardRenderer {
    player: Player,

    x: i32,
    y: i32,

    size: i32
}

impl BoardRenderer {
    pub fn new(x: i32, y: i32, size: i32, player: Player) -> Self {
        Self {
            x,
            y,
            size,
            player
        }
    }

    pub fn set_player(&mut self, player: Player) {
        self.player = player;
    }

    pub fn draw_board(&self) {
        
    }
}
