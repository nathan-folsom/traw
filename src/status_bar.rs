use crossterm::{cursor, terminal};

use crate::{
    draw::{DrawSticky, Point},
    mode::Mode,
};

#[derive(Default)]
pub struct StatusBar {
    mode_text: &'static str,
    cursor_text: String,
}

const NORMAL: &str = "Normal";
const DRAW: &str = "Draw";
const TEXT: &str = "Text";
const ARROW: &str = "Arrow";

impl StatusBar {
    pub fn update(&mut self, mode: &Mode) -> std::io::Result<()> {
        match mode {
            Mode::Normal => {
                self.mode_text = NORMAL;
            }
            Mode::DrawRectangle(_) => {
                self.mode_text = DRAW;
            }
            Mode::DrawArrow(_) => {
                self.mode_text = ARROW;
            }
            Mode::Text(_) => {
                self.mode_text = TEXT;
            }
        }

        let (c_x, c_y) = cursor::position()?;
        self.cursor_text = format!("{}:{}", c_x, c_y);

        Ok(())
    }
}

impl DrawSticky for StatusBar {
    fn draw(&self) -> std::io::Result<Vec<Point<u16>>> {
        let (w, h) = terminal::size()?;
        let mut row = vec![];

        let cursor_text_length = self.cursor_text.chars().count();

        for x in 0..w {
            let mut next_char = ' ';

            if let Some(c) = self.mode_text.chars().nth(x as usize) {
                next_char = c;
            }

            let distance_from_end = (w as usize).abs_diff(x as usize);

            if distance_from_end <= cursor_text_length {
                if let Some(c) = self
                    .cursor_text
                    .chars()
                    .nth(distance_from_end.abs_diff(cursor_text_length))
                {
                    next_char = c;
                }
            }

            row.push(Point {
                x,
                y: h - 11,
                character: next_char,
                foreground: crate::draw::Color::Empty,
                background: crate::draw::Color::EmptyBackground,
            });
        }

        Ok(row)
    }
}
