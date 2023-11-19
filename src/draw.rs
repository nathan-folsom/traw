use std::io::stdout;

use crossterm::{cursor, queue, style::Print};

pub trait Draw {
    fn draw(&self) -> std::io::Result<Vec<(i32, i32, char)>>;
}

pub trait Clear {
    fn clear(&self) -> std::io::Result<Vec<(i32, i32)>>;
}

pub trait DrawSticky {
    fn draw(&self) -> std::io::Result<Vec<(u16, u16, char)>>;
}

pub struct Renderer {
    // x: u32,
    // y: u32,
    // width: u16,
    // height: u16,
    state: Vec<Vec<char>>,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        let mut initial_state = vec![];
        for _ in 0..width {
            let mut cols = vec![];
            for _ in 0..height {
                cols.push(' ');
            }
            initial_state.push(cols);
        }

        Self {
            //         x: 0,
            //         y: 0,
            //         width,
            //         height,
            state: initial_state,
        }
    }

    pub fn render(&mut self, shape: &impl Draw) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor::position()?;
        let points = shape.draw()?;

        for (x, y, c) in points {
            self.draw_at(x, y, c)?;
        }

        queue!(stdout(), cursor::MoveTo(cursor_x, cursor_y),)?;

        Ok(())
    }

    pub fn render_sticky(&mut self, shape: &impl DrawSticky) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor::position()?;
        let points = shape.draw()?;

        for (x, y, c) in points {
            self.draw_at(x as i32, y as i32, c)?;
        }

        queue!(stdout(), cursor::MoveTo(cursor_x, cursor_y),)?;

        Ok(())
    }

    pub fn clear(&mut self, shape: &impl Clear) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor::position()?;
        let points = shape.clear()?;

        for (x, y) in points {
            self.draw_at(x, y, ' ')?;
        }

        queue!(stdout(), cursor::MoveTo(cursor_x, cursor_y),)?;

        Ok(())
    }

    fn draw_at(&mut self, x: i32, y: i32, c: char) -> std::io::Result<()> {
        let current_char = self.state[x as usize][y as usize];

        if current_char != c {
            queue!(stdout(), cursor::MoveTo(x as u16, y as u16), Print(c))?;
            self.state[x as usize][y as usize] = c;
        }

        Ok(())
    }
}
