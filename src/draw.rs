use std::io::stdout;

use crossterm::{cursor, queue, style::Print, terminal};

pub trait Draw {
    fn draw(&self) -> std::io::Result<Vec<(i32, i32, char)>>;
}

pub struct Renderer {
    x: u32,
    y: u32,
    width: u16,
    height: u16,
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
            x: 0,
            y: 0,
            width,
            height,
            state: initial_state,
        }
    }

    pub fn render(&mut self, shape: &impl Draw) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor::position()?;
        let points = shape.draw()?;

        for point in points {
            let current_char = self.state[point.0 as usize][point.1 as usize];

            if current_char != point.2 {
                queue!(
                    stdout(),
                    cursor::MoveTo(point.0 as u16, point.1 as u16),
                    Print(point.2)
                )?;
                self.state[point.0 as usize][point.1 as usize] = point.2;
            }
        }

        queue!(stdout(), cursor::MoveTo(cursor_x, cursor_y),)?;

        Ok(())
    }
}
