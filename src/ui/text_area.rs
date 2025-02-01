use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};

pub struct TextArea {
    x: i32,
    y: i32,

    font_size: i32,

    text_line: i32,
}

impl TextArea {
    pub fn new(x: i32, y: i32, font_size: i32) -> Self {
        Self {
            x,
            y,
            font_size,
            text_line: 0
        }
    }

    pub fn draw_line(&mut self, draw_handle: &mut RaylibDrawHandle, text: &str) {
        let y = self.y + self.text_line * self.font_size;
        draw_handle.draw_text(text, self.x, y, self.font_size, Color::WHITE);

        self.text_line += 1;
    }

    pub fn skip_line(&mut self) {
        self.text_line += 1;
    }

    pub fn reset(&mut self) {
        self.text_line = 0;
    }
}