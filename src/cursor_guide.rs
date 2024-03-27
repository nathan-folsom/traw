use crate::{
    characters::{HORIZONTAL_BAR, VERTICAL_BAR},
    debug_panel::debug,
    draw::{Color, Draw, Point},
};
use crossterm::cursor;

pub struct CursorGuide {
    points: Vec<(i32, i32)>,
}

impl CursorGuide {
    pub fn new(shapes: &[impl GuidePoint]) -> Self {
        Self {
            points: shapes
                .iter()
                .flat_map(|s| s.get_intersection_points())
                .collect(),
        }
    }

    fn get_point(x: i32, y: i32, character: char) -> Point<i32> {
        Point {
            x,
            y,
            character,
            foreground: Color::Guide,
            background: Color::EmptyBackground,
        }
    }
}

impl Draw for CursorGuide {
    fn draw(&self) -> std::io::Result<Vec<Point<i32>>> {
        let (cursor_x, cursor_y) = cursor::position()?;
        let c_x = cursor_x as i32;
        let c_y = cursor_y as i32;
        let mut points = vec![];
        self.points.iter().for_each(|(_x, _y)| {
            let x = *_x;
            let y = *_y;
            match (c_x == x, c_y == y) {
                (true, false) => {
                    for i in (y.min(c_y) + 1)..y.max(c_y) {
                        points.push(Self::get_point(x, i, VERTICAL_BAR));
                    }
                }
                (false, true) => {
                    for i in (x.min(c_x) + 1)..x.max(c_x) {
                        points.push(Self::get_point(i, y, HORIZONTAL_BAR));
                    }
                }
                _ => {}
            }
        });
        Ok(points)
    }
}

/// Used for showing guides when the cursor lines up with an object in one dimension
pub trait GuidePoint {
    fn get_intersection_points(&self) -> Vec<(i32, i32)>;
}
