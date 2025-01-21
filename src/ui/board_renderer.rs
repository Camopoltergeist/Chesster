use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};

use crate::player::Player;

pub struct BoardRenderer {
    player: Player,

    x: i32,
    y: i32,

    size: i32,
    margin: i32,

    dark_color: Color,
    light_color: Color,

    font_size: i32,
}

impl BoardRenderer {
    pub fn new(x: i32, y: i32, size: i32, margin: i32, player: Player) -> Self {
        Self {
            x,
            y,
            size,
            margin,
            player,
            dark_color: Color::BLACK,
            light_color: Color::WHITE,
            font_size: 16,
        }
    }

    pub fn set_player(&mut self, player: Player) {
        self.player = player;
    }

    pub fn draw_board(&self, draw_handle: &mut RaylibDrawHandle) {
        self.draw_tiles(draw_handle);
        self.draw_ranks(draw_handle);
    }

    fn draw_tiles(&self, draw_handle: &mut RaylibDrawHandle) {
        let start_x = self.x + self.margin;
        let start_y = self.y + self.margin;

        let tile_size = self.tile_size();

        for i in 0..8 {
            for j in 0..8 {
                let color = if (i + j) % 2 == 0 { self.light_color } else { self.dark_color };

                let pos = self.get_tile_pos(i, j);

                let x = pos.0 + start_x;
                let y = pos.1 + start_y;

                draw_handle.draw_rectangle(x, y, tile_size, tile_size, color);
            }
        }
    }

    fn draw_ranks(&self, draw_handle: &mut RaylibDrawHandle) {
        let start_x = self.margin / 2;
        let start_y = self.margin;

        let tile_size = self.tile_size();

        for i in 0..8 {
            let rank = if let Player::Black = self.player { i + 1 } else { 8 - i };
            let pos = self.get_tile_pos(i, 0);
            let rank_string = rank.to_string();
            let text_width = draw_handle.measure_text(&rank_string, self.font_size);

            let x = start_x - text_width / 2;
            let y = start_y + pos.1 + tile_size / 2 - self.font_size / 2;

            draw_handle.draw_text(&rank_string, x, y, self.font_size, Color::WHITE);
        }
    }

    fn tile_size(&self) -> i32 {
        let available_area = self.size - self.margin * 2;

        available_area / 8
    }

    fn get_tile_pos(&self, rank: i32, column: i32) -> (i32, i32) {
        let tile_size = self.tile_size();

        let x = column * tile_size;
        let y = rank * tile_size;

        return (x, y);
    }
}
