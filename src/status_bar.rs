use crossterm::terminal;

use crate::{draw::DrawSticky, mode::Mode};

#[derive(Default)]
pub struct StatusBar {
    mode_text: &'static str,
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

        Ok(())
    }
}

impl DrawSticky for StatusBar {
    fn draw(&self) -> std::io::Result<Vec<(u16, u16, char)>> {
        let (w, h) = terminal::size()?;
        let mut row = vec![];

        let mut mode_chars = self.mode_text.chars();

        for x in 0..w {
            let found_char = mode_chars.next();

            let mut next_char = ' ';

            if let Some(c) = found_char {
                next_char = c;
            }

            row.push((x, h - 1, next_char));
        }

        Ok(row)
    }
}
