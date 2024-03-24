use std::io::stdout;

use crossterm::{cursor, queue};

pub struct MotionState {
    count: Vec<char>,
}

impl MotionState {
    pub fn new() -> Self {
        Self { count: vec![] }
    }
    pub fn handle_motions(&mut self, key: char) -> std::io::Result<()> {
        match key {
            'h' => {
                queue!(stdout(), cursor::MoveLeft(self.get_count()))?;
            }
            'j' => {
                queue!(stdout(), cursor::MoveDown(self.get_count()))?;
            }
            'k' => {
                queue!(stdout(), cursor::MoveUp(self.get_count()))?;
            }
            'l' => {
                queue!(stdout(), cursor::MoveRight(self.get_count()))?;
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.count.push(key),
            _ => {}
        }

        Ok(())
    }
    fn get_count(&mut self) -> u16 {
        let chars = std::mem::take(&mut self.count);
        chars.iter().collect::<String>().parse::<u16>().unwrap_or(1)
    }
}
