use std::collections::HashMap;

use raylib::{color::Color, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle}, texture::Texture2D};

use crate::{board::bitboard::Bitboard, player::Player};

use super::texture::PieceTexture;

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

    textures: HashMap<PieceTexture, Texture2D>,
}

impl BoardRenderer {
    pub fn new(x: i32, y: i32, size: i32, margin: i32, player: Player, piece_textures: HashMap<PieceTexture, Texture2D>) -> Self {
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
            bitboard_off_color: Color { r: 0, g: 0, b: 255, a: 127 },
            textures: piece_textures
        }
    }

    fn draw_piece(&self, draw_handle: &mut RaylibDrawHandle, piece_texture: PieceTexture, column: i32, rank: i32) {
        let pos = self.get_tile_pixel_pos(rank, column);
        let tile_size = self.tile_size();

        let x = pos.0;
        let y = pos.1;

        let texture = self.textures.get(&piece_texture).expect("invalid piece texture");

        let scale = tile_size as f32 / texture.height as f32;

        draw_handle.draw_texture_ex(texture, Vector2::new(x as f32, y as f32), 0.0, scale, Color::WHITE);
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
        for bit_offset in 0..64 {
            let bit = (self.bitboard & 1 << bit_offset) != 0;
            let color = if bit { self.bitboard_on_color } else { self.bitboard_off_color };

            let (column, rank) = Bitboard::bit_offset_to_coordinates(bit_offset);

            let pos = self.get_tile_pixel_pos(column, rank);
            let tile_size = self.tile_size();

            draw_handle.draw_rectangle(pos.0, pos.1, tile_size, tile_size, color);
        }
    }

    pub fn set_player(&mut self, player: Player) {
        self.player = player;
    }

    pub fn draw_board(&self, draw_handle: &mut RaylibDrawHandle) {
        self.draw_tiles(draw_handle);
        self.draw_ranks(draw_handle);
        self.draw_columns(draw_handle);
        self.draw_piece(draw_handle, PieceTexture::new(Player::Black, crate::piece::Piece::Pawn), 1, 1);

        if self.draw_bitboard {
            self.draw_bitboard_overlay(draw_handle);
        }
    }

    fn draw_tiles(&self, draw_handle: &mut RaylibDrawHandle) {
        let tile_size = self.tile_size();

        for i in 0..8 {
            for j in 0..8 {
                let color = if (i + j) % 2 == 0 { self.dark_color } else { self.light_color };

                let pos = self.get_tile_pixel_pos(i, j);

                draw_handle.draw_rectangle(pos.0, pos.1, tile_size, tile_size, color);
            }
        }
    }

    fn get_rank_label_x(&self) -> i32 {
        self.x + self.margin / 2
    }

    fn get_column_label_y(&self) -> i32 {
        self.y + self.size - self.margin / 2
    }

    fn get_rank_centered_y(&self, rank: i32) -> i32 {
        let tile_size = self.tile_size();
        self.y + tile_size * rank + tile_size / 2 + self.margin
    }

    fn get_column_centered_x(&self, column: i32) -> i32 {
        let tile_size = self.tile_size();
        self.x + self.margin + tile_size * column + tile_size / 2
    }

    fn draw_ranks(&self, draw_handle: &mut RaylibDrawHandle) {
        let rank_label_x = self.get_rank_label_x();

        for i in 0..8 {
            let rank = if let Player::Black = self.player { i + 1 } else { 8 - i };
            let rank_string = rank.to_string();
            
            let text_width = draw_handle.measure_text(&rank_string, self.font_size);

            let x = rank_label_x - text_width / 2;
            let y = self.get_rank_centered_y(i) - self.font_size / 2;

            draw_handle.draw_text(&rank_string, x, y, self.font_size, Color::WHITE);
        }
    }

    fn draw_columns(&self, draw_handle: &mut RaylibDrawHandle) {
        const COLUMNS: &str = "ABCDEFGH";

        let column_label_y = self.get_column_label_y();

        for i in 0..8 {
            let column = if let Player::Black = self.player { 7 - i } else { i };
            let column_string = COLUMNS.chars().nth(column as usize).expect("failed to get column letter").to_string();

            let text_width = draw_handle.measure_text(&column_string, self.font_size);

            let x = self.get_column_centered_x(i) - text_width / 2;
            let y = column_label_y - self.font_size / 2;

            draw_handle.draw_text(&column_string, x, y, self.font_size, Color::WHITE);
        }
    }

    fn tile_size(&self) -> i32 {
        let available_area = self.size - self.margin * 2;

        available_area / 8
    }

    fn get_tile_pixel_pos(&self, column: i32, rank: i32) -> (i32, i32) {
        let flipped = self.player == Player::Black;

        let tile_x = if flipped { 7 - column } else { column };
        let tile_y = if flipped { rank } else { 7 - rank };
        
        let tile_size = self.tile_size();

        let pixel_x = self.margin + self.x + tile_x * tile_size;
        let pixel_y = self.margin + self.y + tile_y * tile_size;

        return (pixel_x, pixel_y);
    }
}
