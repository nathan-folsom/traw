use crossterm::terminal;

use crate::draw::{Color, Draw, Point};

pub struct GridBackground {}

impl GridBackground {
    pub fn new() -> Self {
        Self {}
    }
}

impl Draw for GridBackground {
    fn draw(&self) -> std::io::Result<Vec<crate::draw::Point<i32>>> {
        let (w, h) = terminal::size()?;
        let mut points = vec![];
        for x in 0..w {
            for y in 0..h {
                if (x % 12 == 0 && y % 6 == 0) || ((x + 6) % 12 == 0 && (y + 3) % 6 == 0) {
                    points.push(
                        Point {
                            x,
                            y,
                            character: '*',
                            foreground: Color::Grid,
                            background: Color::EmptyBackground,
                        }
                        .into(),
                    )
                }
            }
        }
        Ok(points)
    }
}
