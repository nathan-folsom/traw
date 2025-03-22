use crossterm::terminal;

use crate::{
    cursor::cursor_position,
    draw::{DrawSticky, Point},
    mode::Mode,
    util::Vec2,
};

#[derive(Default)]
pub struct StatusBar {
    mode_text: &'static str,
    cursor_text: String,
    y: u16,
}

const NORMAL: &str = "Normal";
const DRAW: &str = "Draw";
const TEXT: &str = "Text";
const ARROW: &str = "Arrow";
const SELECT: &str = "Select";

impl StatusBar {
    pub fn new(mode: &Mode, y_offset: u16) -> Self {
        let mode_text = match mode {
            Mode::Normal => NORMAL,
            Mode::DrawRectangle(_, _) => DRAW,
            Mode::DrawArrow(_) => ARROW,
            Mode::Text(_) => TEXT,
            Mode::Select(_) => SELECT,
        };

        let position = cursor_position();
        let cursor_text = format!("{}:{}", position.x, position.y);
        let y = y_offset + 1;

        Self {
            mode_text,
            cursor_text,
            y,
        }
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
                origin: Vec2 { x, y: h - self.y },
                character: next_char,
                foreground: crate::draw::Color::Empty,
                background: crate::draw::Color::EmptyBackground,
            });
        }

        Ok(row)
    }
}
