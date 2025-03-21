use std::collections::HashMap;

use raylib::{color::Color, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle}, texture::Texture2D};

use crate::{board::{bitboard::Bitboard, board::Board, moove::Move, tile_position::TilePosition}, player::Player};

use super::texture::PieceTexture;

/// Renders a chess board with labels and all the pieces on it. Also includes bitboard overlay
pub struct BoardRenderer {
    /// Determines the player whose perspective is used to render the board
    player: Player,

    /// X pixel offset on screen
    x: i32,
    /// Y pixel offset on screen
    y: i32,

    /// Entire size of the board including margins
    size: i32,
    /// Margin of the board used to render labels
    margin: i32,

    /// Color of the dark squares of the board
    dark_color: Color,
    /// Color of the light squares of the board
    light_color: Color,

    /// Font size of the labels
    font_size: i32,

    /// Bitboard used to draw overlay. If None, no overlay will be drawn.
    bitboard: Option<Bitboard>,

    /// Color used to draw ON bits on the overlay 
    bitboard_on_color: Color,
    /// Color used to draw OFF bits on the overlay
    bitboard_off_color: Color,

    /// Map of chess piece textures
    piece_textures: HashMap<PieceTexture, Texture2D>,

    /// Texture used to indicate possible moves
    circle_texture: Texture2D,

    /// Board state used to draw pieces
    board: Board,

    /// Tile which is highlighted separately
    highlighted_tile: Option<TilePosition>,

    /// Vector of moves to be shown on board
    moves: Vec<Move>,

    /// Color of the legal move indicator circles
    move_color: Color,

    /// Color of capturing moves
    capturing_move_color: Color,

    last_move: Option<Move>,
}

impl BoardRenderer {
    pub fn new(x: i32, y: i32, size: i32, margin: i32, player: Player, piece_textures: HashMap<PieceTexture, Texture2D>, circle_texture: Texture2D) -> Self {
        Self {
            x,
            y,
            size,
            margin,
            player,
            dark_color: Color { r: 0, g: 123, b: 255, a: 255 },
            light_color: Color { r: 163, g: 227, b: 255, a: 255 },
            font_size: 16,
            bitboard: None,
            bitboard_on_color: Color { r: 255, g: 0, b: 0, a: 127 },
            bitboard_off_color: Color { r: 0, g: 0, b: 255, a: 127 },
            piece_textures,
            circle_texture,
            board: Board::empty(),
            highlighted_tile: None,
            moves: Vec::new(),
            move_color: Color { r: 0, g: 0, b: 0, a: 127 },
            capturing_move_color: Color { r: 255, g: 0, b: 0, a: 127 },
            last_move: None,
        }
    }

    pub fn set_size(&mut self, size: i32) {
        self.size = size;
    }

    /// Draws a specified piece on the specified tile
    fn draw_piece(&self, draw_handle: &mut RaylibDrawHandle, piece_texture: PieceTexture, tile_pos: TilePosition) {
        let pos = self.get_tile_pixel_pos(tile_pos);
        let tile_size = self.tile_size();

        let x = pos.0;
        let y = pos.1;

        let texture = self.piece_textures.get(&piece_texture).expect("invalid piece texture");

        let scale = tile_size as f32 / texture.height as f32;

        draw_handle.draw_texture_ex(texture, Vector2::new(x as f32, y as f32), 0.0, scale, Color::WHITE);
    }

    /// Sets bitboard used to draw overlay. None disables overlay.
    #[allow(dead_code)]
    pub fn set_bitboard_overlay(&mut self, bitboard: Option<Bitboard>) {
        self.bitboard = bitboard;
    }

    /// If bitboard is Some, draw a representation of it as overlay.
    fn draw_bitboard_overlay(&self, draw_handle: &mut RaylibDrawHandle) {
        if let Some(bitboard) = self.bitboard {
            for bit_offset in 0..64 {
                let bit = bitboard.check_bit(bit_offset);
                let color = if bit { self.bitboard_on_color } else { self.bitboard_off_color };
    
                let tile_pos = TilePosition::from_bit_offset(bit_offset);

                if let Some(highlight) = self.highlighted_tile {
                    if highlight == tile_pos {
                        continue;
                    }
                }
    
                let pos = self.get_tile_pixel_pos(tile_pos);
                let tile_size = self.tile_size();
    
                draw_handle.draw_rectangle(pos.0, pos.1, tile_size, tile_size, color);
            }
        }
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn margin(&self) -> i32 {
        self.margin
    }

    /// Flips the perspective to the other player
    pub fn swap_player(&mut self) {
        self.player = self.player.opposite();
    }

    pub fn set_highlighted_tile(&mut self, tile: Option<TilePosition>) {
        self.highlighted_tile = tile;
    }

    pub fn set_legal_moves(&mut self, legal_moves: Vec<Move>) {
        self.moves = legal_moves;
    }

    fn flipped(&self) -> bool {
        self.player == Player::Black
    }

    pub fn get_tile_from_pixel_pos(&self, pos: Vector2) -> Option<TilePosition> {
        let mouse_x = pos.x as i32;
        let mouse_y = pos.y as i32;

        let tile_size = self.tile_size();
        let total_tile_size = tile_size * 8 - 1;

        if mouse_x < self.x + self.margin ||
            mouse_x > self.x + self.margin + total_tile_size ||
            mouse_y < self.y + self.margin ||
            mouse_y > self.y + self.margin + total_tile_size {
            
            return None;
        };

        let column = if self.flipped() { 7 - (mouse_x - self.margin) / tile_size } else { (mouse_x - self.margin) / tile_size };
        let rank = if self.flipped() { (mouse_y - self.margin) / tile_size } else { 7 - (mouse_y - self.margin) / tile_size };

        return Some(TilePosition::new(column as u32, rank as u32));
    }

    /// Draws the board on screen
    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        self.draw_tiles(draw_handle);
        self.draw_last_move(draw_handle);
        self.draw_highlighted_tile(draw_handle);
        self.draw_ranks(draw_handle);
        self.draw_columns(draw_handle);
        self.draw_board_pieces(draw_handle);
        self.draw_legal_moves(draw_handle);

        self.draw_bitboard_overlay(draw_handle);
    }

    fn draw_last_move(&self, draw_handle: &mut RaylibDrawHandle) {
        let tile_size = self.tile_size();

        if let Some(last_move) = &self.last_move {
            let from_pos = self.get_tile_pixel_pos(last_move.from_position());
            let to_pos = self.get_tile_pixel_pos(last_move.to_position());

            draw_handle.draw_rectangle(from_pos.0, from_pos.1, tile_size, tile_size, Color::ROSYBROWN);
            draw_handle.draw_rectangle(to_pos.0, to_pos.1, tile_size, tile_size, Color::ROSYBROWN);
        }
    }

    pub fn set_last_move(&mut self, last_move: Option<Move>) {
        self.last_move = last_move;
    }

    /// Sets the current board to be drawn. Set to None to disable pieces.
    pub fn set_board(&mut self, board: &Board) {
        self.board = board.clone();
    }

    fn draw_board_pieces(&self, draw_handle: &mut RaylibDrawHandle) {
        for bit_offset in 0..64 {
            let tile_pos = TilePosition::from_bit_offset(bit_offset);

            let player_piece_opt = self.board.get_piece(tile_pos);

            if let Some(piece) = player_piece_opt {
                self.draw_piece(draw_handle, PieceTexture::new(piece), tile_pos);
            }
        }
    }

    fn draw_tiles(&self, draw_handle: &mut RaylibDrawHandle) {
        let tile_size = self.tile_size();

        for rank in 0..8 {
            for column in 0..8 {
                let color = if (rank + column) % 2 == 0 { self.dark_color } else { self.light_color };

                let pos = self.get_tile_pixel_pos(TilePosition::new(column, rank));
                draw_handle.draw_rectangle(pos.0, pos.1, tile_size, tile_size, color);
            }
        }
    }

    fn draw_highlighted_tile(&self, draw_handle: &mut RaylibDrawHandle) {
        let tile_size = self.tile_size();

        if let Some(highlight) = self.highlighted_tile {
            let pos = self.get_tile_pixel_pos(highlight);

            draw_handle.draw_rectangle(pos.0, pos.1, tile_size, tile_size, Color::GREEN);
        }
    }

    fn draw_legal_moves(&self, draw_handle: &mut RaylibDrawHandle) {
        let tile_size = self.tile_size();
        let texture_size = self.circle_texture.height;

        let circle_diameter = tile_size as f32 / 3.0;

        let scale = circle_diameter / texture_size as f32;

        for m in &self.moves {
            let is_capturing = self.board.get_piece(m.to_position()).is_some() || matches!(m, Move::EnPassant(_));

            let color = if is_capturing { self.capturing_move_color } else { self.move_color };

            let (x, y) = self.get_tile_pixel_pos(m.to_position());

            draw_handle.draw_texture_ex(&self.circle_texture, Vector2::new(x as f32 + circle_diameter, y as f32 + circle_diameter), 0.0, scale, color);
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

    fn get_tile_pixel_pos(&self, tile_pos: TilePosition) -> (i32, i32) {
        let flipped = self.player == Player::Black;

        let tile_x = (if flipped { 7 - tile_pos.column() } else { tile_pos.column() } as i32);
        let tile_y = (if flipped { tile_pos.rank() } else { 7 - tile_pos.rank() } as i32);
        
        let tile_size = self.tile_size();

        let pixel_x = self.margin + self.x + tile_x * tile_size;
        let pixel_y = self.margin + self.y + tile_y * tile_size;

        return (pixel_x, pixel_y);
    }
}
