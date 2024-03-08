use serde::{Deserialize, Serialize};
use std::io::stdout;

use crossterm::{
    cursor::{self},
    queue,
};

use crate::{
    characters::{
        CORNER_1_ROUNDED, CORNER_2_ROUNDED, CORNER_3_ROUNDED, CORNER_4_ROUNDED, HORIZONTAL_BAR,
        VERTICAL_BAR,
    },
    draw::{Color, Draw, Intersection, Point},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn on_char(&mut self, key: char) -> std::io::Result<()> {
        self.text.push(key);
        let (next_x, next_y) = self.get_inner_cursor_position();
        queue!(stdout(), cursor::MoveTo(next_x as u16, next_y as u16))?;
        Ok(())
    }

    pub fn on_backspace(&mut self) -> std::io::Result<()> {
        self.text.pop();
        let (next_x, next_y) = self.get_inner_cursor_position();
        queue!(stdout(), cursor::MoveTo(next_x as u16, next_y as u16))?;
        Ok(())
    }

    pub fn get_inner_cursor_position(&self) -> (i32, i32) {
        if self.width < 3 || self.height < 3 {
            return (self.x, self.y);
        }
        let text_width = self.width - 2;
        let col = self.text.len() as i32 % text_width;
        let row = self.text.len() as i32 / text_width;
        let cursor_x = self.x + 1 + col;
        let cursor_y = self.y + 1 + row;
        (cursor_x, cursor_y)
    }
}

impl Draw for Rectangle {
    fn draw(&self, hover: bool) -> std::io::Result<Vec<Point<i32>>> {
        let mut points = vec![];
        let foreground = Color::Border;
        let background = {
            if hover {
                Color::BorderBackgroundHover
            } else {
                Color::BorderBackground
            }
        };

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
                    to_draw = CORNER_3_ROUNDED;
                } else if is_first_row && is_last_col {
                    to_draw = CORNER_4_ROUNDED;
                } else if is_last_row && is_last_col {
                    to_draw = CORNER_1_ROUNDED;
                } else if is_last_row && is_first_col {
                    to_draw = CORNER_2_ROUNDED;
                } else if is_first_row || is_last_row {
                    to_draw = HORIZONTAL_BAR;
                } else if is_first_col || is_last_col {
                    to_draw = VERTICAL_BAR;
                }

                points.push(Point {
                    x: self.x + x,
                    y: self.y + y,
                    character: to_draw,
                    foreground,
                    background,
                });
            }
        }

        match self.shrink {
            Shrink::X => {
                for y in 0..self.height {
                    points.push(Point {
                        x: self.x + self.width,
                        y: self.y + y,
                        character: ' ',
                        foreground: Color::Empty,
                        background: Color::EmptyBackground,
                    });
                }
            }
            Shrink::Y => {
                for x in 0..self.width {
                    points.push(Point {
                        x: self.x + x,
                        y: self.y + self.height,
                        character: ' ',
                        foreground: Color::Empty,
                        background: Color::EmptyBackground,
                    });
                }
            }
            _ => {}
        }

        Ok(points)
    }

    fn get_intersection(&self) -> std::io::Result<crate::draw::Intersection> {
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
            return Ok(Intersection::None);
        }

        let cursor_on_top_border = c_y == y_0;
        let cursor_on_bottom_border = c_y == y_1;
        let cursor_on_left_border = c_x == x_0;
        let cursor_on_right_border = c_x == x_1;

        if (cursor_on_right_border || cursor_on_left_border)
            && (cursor_on_top_border || cursor_on_bottom_border)
        {
            return Ok(Intersection::Edge(crate::draw::EdgeIntersection::Corner));
        }

        if cursor_on_top_border
            || cursor_on_right_border
            || cursor_on_bottom_border
            || cursor_on_left_border
        {
            return Ok(Intersection::Edge(crate::draw::EdgeIntersection::Side));
        }

        Ok(Intersection::Inner)
    }

    fn clear(&self) -> std::io::Result<Vec<(i32, i32)>> {
        let mut points = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                points.push((x + self.x, y + self.y))
            }
        }
        Ok(points)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
enum Shrink {
    X,
    Y,
    None,
}

#[cfg(test)]
mod test {
    use crate::draw::Draw;

    use super::Rectangle;

    #[test]
    fn should_clear() {
        let rect = Rectangle {
            x: 5,
            y: 5,
            width: 2,
            height: 2,
            shrink: super::Shrink::None,
            text: vec![],
        };
        let clear = rect.clear().unwrap();
        let expected_clear = vec![(5, 5), (5, 6), (6, 5), (6, 6)];
        assert_eq!(expected_clear.len(), clear.len());
        assert_eq!(clear, expected_clear);
    }

    #[test]
    fn should_get_cursor_position_when_editing_text() {
        let rect = Rectangle {
            x: 5,
            y: 5,
            width: 4,
            height: 4,
            shrink: super::Shrink::None,
            text: vec!['0', '1', '2'],
        };
        let pos = rect.get_inner_cursor_position();
        let expected = (7, 7);
        assert_eq!(pos, expected);
    }
}
