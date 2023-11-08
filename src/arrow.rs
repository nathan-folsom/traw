use crossterm::cursor;

use crate::draw::Draw;

pub struct Arrow {
    pub points: Vec<(i32, i32)>,
}

impl Arrow {
    pub fn update(&mut self) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor::position()?;
        self.points.push((cursor_x as i32, cursor_y as i32));
        Ok(())
    }
}

impl Draw for Arrow {
    fn draw(&self) -> std::io::Result<Vec<(i32, i32, char)>> {
        Ok(self.points.iter().map(|(x, y)| (*x, *y, '*')).collect())
    }
}
