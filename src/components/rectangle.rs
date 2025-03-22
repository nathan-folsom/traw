use serde::{Deserialize, Serialize};
use std::cmp::Ordering::{Greater, Less};

use crate::{
    characters::{
        CORNER_1_ROUNDED, CORNER_2_ROUNDED, CORNER_3_ROUNDED, CORNER_4_ROUNDED, HORIZONTAL_BAR,
        VERTICAL_BAR,
    },
    cursor::{cursor_position, set_position},
    cursor_guide::GuidePoint,
    draw::{Color, CursorIntersect, Draw, EdgeIntersection::Corner, Intersection, Point},
    mode::Anchor,
    shape_id::generate_shape_id,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub text: Vec<char>,
    pub shape_id: u32,
}

impl Rectangle {
    pub fn new_at(x: i32, y: i32) -> Rectangle {
        Self {
            x,
            y,
            width: 1,
            height: 1,
            text: vec![],
            shape_id: generate_shape_id(),
        }
    }

    pub fn on_char(&mut self, key: char) -> std::io::Result<()> {
        self.text.push(key);
        let (next_x, next_y) = self.get_inner_cursor_position();
        set_position((next_x as u16, next_y as u16).into());
        Ok(())
    }

    pub fn on_backspace(&mut self) -> std::io::Result<()> {
        self.text.pop();
        let (next_x, next_y) = self.get_inner_cursor_position();
        set_position((next_x as u16, next_y as u16).into());
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
    fn draw(&self) -> std::io::Result<Vec<Point<i32>>> {
        let mut points = vec![];
        let foreground = Color::Border;
        let background = Color::BorderBackground;

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

        Ok(points)
    }
}

impl CursorIntersect for Rectangle {
    fn get_intersection(&self, x: &i32, y: &i32) -> Intersection {
        let c_x = *x;
        let c_y = *y;
        let y_0 = self.y;
        let x_0 = self.x;
        let y_1 = self.y + self.height - 1;
        let x_1 = self.x + self.width - 1;

        let cursor_x_in_rectangle = c_x >= x_0 && c_x <= x_1;
        let cursor_y_in_rectangle = c_y >= y_0 && c_y <= y_1;

        if !cursor_x_in_rectangle || !cursor_y_in_rectangle {
            return Intersection::None;
        }

        let cursor_on_top_border = c_y == y_0;
        let cursor_on_bottom_border = c_y == y_1;
        let cursor_on_left_border = c_x == x_0;
        let cursor_on_right_border = c_x == x_1;

        if cursor_on_right_border && cursor_on_top_border {
            return Intersection::Edge(Corner(Some(Anchor::TopRight)));
        } else if cursor_on_right_border && cursor_on_bottom_border {
            return Intersection::Edge(Corner(Some(Anchor::BottomRight)));
        } else if cursor_on_left_border && cursor_on_top_border {
            return Intersection::Edge(Corner(Some(Anchor::TopLeft)));
        } else if cursor_on_left_border && cursor_on_bottom_border {
            return Intersection::Edge(Corner(Some(Anchor::BottomLeft)));
        }

        if cursor_on_top_border
            || cursor_on_right_border
            || cursor_on_bottom_border
            || cursor_on_left_border
        {
            return Intersection::Edge(crate::draw::EdgeIntersection::Side);
        }

        Intersection::Inner
    }
}

impl Drag for Rectangle {
    fn rect(&mut self) -> (&mut i32, &mut i32, &mut i32, &mut i32) {
        (&mut self.x, &mut self.y, &mut self.width, &mut self.height)
    }
}

pub trait Drag {
    fn rect(&mut self) -> (&mut i32, &mut i32, &mut i32, &mut i32);

    fn drag_corner(&mut self, anchor: &mut Anchor) -> std::io::Result<()> {
        let position = cursor_position();
        let cursor_x = position.x as i32;
        let cursor_y = position.y as i32;
        self.adjust_anchor(anchor, &cursor_x, &cursor_y);
        match anchor {
            Anchor::TopLeft => {
                self.drag_top(cursor_y);
                self.drag_left(cursor_x);
            }
            Anchor::TopRight => {
                self.drag_top(cursor_y);
                self.drag_right(cursor_x);
            }
            Anchor::BottomRight => {
                self.drag_right(cursor_x);
                self.drag_bottom(cursor_y);
            }
            Anchor::BottomLeft => {
                self.drag_bottom(cursor_y);
                self.drag_left(cursor_x);
            }
        }

        Ok(())
    }

    /// Allow dragging to continue even when the cursor passes to the "other side" of
    /// the box border
    fn adjust_anchor(&mut self, anchor: &mut Anchor, cursor_x: &i32, cursor_y: &i32) {
        let (x, y, width, height) = self.rect();
        match anchor {
            Anchor::TopLeft => {
                if cursor_x > x && width == &1 {
                    *anchor = Anchor::TopRight;
                } else if cursor_y > y && height == &1 {
                    *anchor = Anchor::BottomLeft;
                }
            }
            Anchor::TopRight => {
                if cursor_x < x && width == &1 {
                    *anchor = Anchor::TopLeft;
                } else if cursor_y > y && height == &1 {
                    *anchor = Anchor::BottomRight;
                }
            }
            Anchor::BottomLeft => {
                if cursor_x > x && width == &1 {
                    *anchor = Anchor::BottomRight;
                } else if cursor_y < y && height == &1 {
                    *anchor = Anchor::TopLeft;
                }
            }
            Anchor::BottomRight => {
                if cursor_x < x && width == &1 {
                    *anchor = Anchor::BottomLeft;
                } else if cursor_y < y && height == &1 {
                    *anchor = Anchor::TopRight;
                }
            }
        }
    }

    fn drag_right(&mut self, cursor_x: i32) {
        let (x, _, width, _) = self.rect();
        *width = cursor_x - *x + 1;
    }

    fn drag_top(&mut self, cursor_y: i32) {
        let (_, y, _, height) = self.rect();
        match cursor_y.cmp(y) {
            Less => *height += 1,
            Greater => *height -= 1,
            _ => {}
        }
        *y = cursor_y;
    }

    fn drag_bottom(&mut self, cursor_y: i32) {
        let (_, y, _, height) = self.rect();
        *height = cursor_y - *y + 1;
    }

    fn drag_left(&mut self, cursor_x: i32) {
        let (x, _, width, _) = self.rect();
        match cursor_x.cmp(x) {
            Less => *width += 1,
            Greater => *width -= 1,
            _ => {}
        }
        *x = cursor_x;
    }
}

impl GuidePoint for Rectangle {
    fn get_intersection_points(&self) -> Vec<(i32, i32)> {
        [
            (self.x, self.y),
            (self.x + self.width - 1, self.y),
            (self.x + self.width - 1, self.y + self.height - 1),
            (self.x, self.y + self.height - 1),
        ]
        .into()
    }
}

#[cfg(test)]
mod test {
    use super::Rectangle;

    #[test]
    fn should_get_cursor_position_when_editing_text() {
        let rect = Rectangle {
            x: 5,
            y: 5,
            width: 4,
            height: 4,
            text: vec!['0', '1', '2'],
            shape_id: 1,
        };
        let pos = rect.get_inner_cursor_position();
        let expected = (7, 7);
        assert_eq!(pos, expected);
    }
}
