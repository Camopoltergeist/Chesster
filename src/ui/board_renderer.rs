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

    draw_bitboard: bool,
    bitboard: u64,

    bitboard_on_color: Color,
    bitboard_off_color: Color,

    
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
            draw_bitboard: false,
            bitboard: 0,
            bitboard_on_color: Color { r: 255, g: 0, b: 0, a: 127 },
            bitboard_off_color: Color { r: 0, g: 0, b: 255, a: 127 }
        }
    }

    pub fn set_bitboard_overlay(&mut self, bitboard: u64) {
        self.bitboard = bitboard;
        self.draw_bitboard = true;
    }

    pub fn clear_bitboard_overlay(&mut self) {
        self.bitboard = 0;
        self.draw_bitboard = false;
    }

    fn draw_bitboard_overlay(&self, draw_handle: &mut RaylibDrawHandle) {
        let start_x = self.margin;
        let start_y = self.margin;

        for bit_offset in 0..64 {
            let bit = (self.bitboard & 1 << bit_offset) != 0;
            let color = if bit { self.bitboard_on_color } else { self.bitboard_off_color };

            let flipped = matches!(self.player, Player::Black);

            let column = if flipped { 7 - bit_offset % 8 } else { bit_offset % 8 };
            let rank = if flipped { bit_offset / 8 } else { 7 - bit_offset / 8 };

            let pos = self.get_tile_pos(rank, column);
            let tile_size = self.tile_size();

            let x = start_x + pos.0;
            let y = start_y + pos.1;

            draw_handle.draw_rectangle(x, y, tile_size, tile_size, color);
        }
    }

    pub fn set_player(&mut self, player: Player) {
        self.player = player;
    }

    pub fn draw_board(&self, draw_handle: &mut RaylibDrawHandle) {
        self.draw_tiles(draw_handle);
        self.draw_ranks(draw_handle);
        self.draw_columns(draw_handle);

        if self.draw_bitboard {
            self.draw_bitboard_overlay(draw_handle);
        }
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
        let start_x = self.x + self.margin / 2;
        let start_y = self.y + self.margin;

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

    fn draw_columns(&self, draw_handle: &mut RaylibDrawHandle) {
        const COLUMNS: &str = "ABCDEFGH";

        let start_x = self.x + self.margin;
        let start_y = self.y + self.size - self.margin / 2;

        let tile_size = self.tile_size();

        for i in 0..8 {
            let column = if let Player::Black = self.player { 7 - i } else { i };
            let pos = self.get_tile_pos(0, i);
            let column_string = COLUMNS.chars().nth(column as usize).expect("failed to get column letter").to_string();
            let text_width = draw_handle.measure_text(&column_string, self.font_size);

            let x = start_x + pos.0 - text_width / 2 + tile_size / 2;
            let y = start_y - self.font_size / 2;

            draw_handle.draw_text(&column_string, x, y, self.font_size, Color::WHITE);
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
