use crossterm::cursor;

use crate::draw::Draw;

#[derive(Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub text: Vec<char>,
    shrink: Shrink,
}

impl Rectangle {
    pub fn new_at(x: i32, y: i32) -> Rectangle {
        Self {
            x,
            y,
            width: 1,
            height: 1,
            text: vec![],
            shrink: Shrink::None,
        }
    }

    pub fn update(&mut self) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor::position()?;

        let next_width = cursor_x as i32 - self.x + 1;
        let next_height = cursor_y as i32 - self.y + 1;

        if next_width < self.width {
            self.shrink = Shrink::X;
        } else if next_height < self.height {
            self.shrink = Shrink::Y;
        }

        self.width = next_width;
        self.height = next_height;

        Ok(())
    }

    pub fn on_char(&mut self, key: char) {
        self.text.push(key);
    }

    pub fn get_intersection(&self) -> std::io::Result<RectangleIntersection> {
        let (cursor_x, cursor_y) = cursor::position()?;

        let c_x = cursor_x as i32;
        let c_y = cursor_y as i32;
        let y_0 = self.y;
        let x_0 = self.x;
        let y_1 = self.y + self.height - 1;
        let x_1 = self.x + self.width - 1;

        let cursor_x_in_rectangle = c_x >= x_0 && c_x <= x_1;
        let cursor_y_in_rectangle = c_y >= y_0 && c_y <= y_1;

        if !cursor_x_in_rectangle || !cursor_y_in_rectangle {
            return Ok(RectangleIntersection::None);
        }

        let cursor_on_top_border = c_y == y_0;
        let cursor_on_bottom_border = c_y == y_1;
        let cursor_on_left_border = c_x == x_0;
        let cursor_on_right_border = c_x == x_1;

        if cursor_on_top_border
            || cursor_on_right_border
            || cursor_on_bottom_border
            || cursor_on_left_border
        {
            return Ok(RectangleIntersection::Edge);
        }

        Ok(RectangleIntersection::Inner)
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

                let char_index = x - 1 + (y - 1) * (self.width - 2);

                let mut to_draw = ' ';

                if char_index >= 0 && (char_index as usize) < self.text.len() {
                    to_draw = self.text[char_index as usize];
                }

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

        match self.shrink {
            Shrink::X => {
                for y in 0..self.height {
                    points.push((self.x + self.width, self.y + y, ' '));
                }
            }
            Shrink::Y => {
                for x in 0..self.width {
                    points.push((self.x + x, self.y + self.height, ' '));
                }
            }
            _ => {}
        }

        Ok(points)
    }
}

#[derive(Debug)]
enum Shrink {
    X,
    Y,
    None,
}

pub enum RectangleIntersection {
    None,
    Edge,
    Inner,
}
