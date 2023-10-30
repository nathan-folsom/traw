use std::io::stdout;

use crossterm::{cursor, queue, style::Print};

use crate::draw::Draw;

#[derive(Default, Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub fn new_at(x: i32, y: i32) -> Rectangle {
        Self {
            x,
            y,
            width: 1,
            height: 1,
        }
    }
}

impl Draw for Rectangle {
    fn draw(&self) -> std::io::Result<Vec<(i32, i32, char)>> {
        let mut points = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let is_first_row = y == 0;
                let is_last_row = y == self.height - 1;
                let is_first_col = x == 0;
                let is_last_col = x == self.width - 1;

                let mut to_draw = ' ';

                if is_first_row && is_first_col {
                    to_draw = '╭';
                } else if is_first_row && is_last_col {
                    to_draw = '╮';
                } else if is_last_row && is_last_col {
                    to_draw = '╯';
                } else if is_last_row && is_first_col {
                    to_draw = '╰';
                } else if is_first_row || is_last_row {
                    to_draw = '─';
                } else if is_first_col || is_last_col {
                    to_draw = '│';
                }

                points.push((self.x + x, self.y + y, to_draw));
            }
        }

        Ok(points)
    }
}
