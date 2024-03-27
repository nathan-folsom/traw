use crate::{
    characters::{HORIZONTAL_BAR, VERTICAL_BAR},
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
        let mut closest = None;
        self.points
            .iter()
            .filter(|(x, y)| matches!((&c_x == x, &c_y == y), (true, false) | (false, true)))
            .for_each(|(x, y)| {
                let diff = c_x.abs_diff(*x).max(c_y.abs_diff(*y));
                match closest {
                    None => {
                        closest = Some(((*x, *y), diff));
                    }
                    Some((_, prev_diff)) => {
                        if prev_diff > diff {
                            closest = Some(((*x, *y), diff));
                        }
                    }
                }
            });
        let mut points = vec![];
        if let Some(((x, y), _)) = closest {
            match (c_x == x, c_y == y) {
                (true, false) => {
                    for i in (y + 1)..c_y {
                        points.push(Self::get_point(x, i, VERTICAL_BAR));
                    }
                }
                (false, true) => {
                    for i in (x + 1)..c_x {
                        points.push(Self::get_point(i, y, HORIZONTAL_BAR));
                    }
                }
                _ => {}
            }
        }
        Ok(points)
    }
}

/// Used for showing guides when the cursor lines up with an object in one dimension
pub trait GuidePoint {
    fn get_intersection_points(&self) -> Vec<(i32, i32)>;
}
