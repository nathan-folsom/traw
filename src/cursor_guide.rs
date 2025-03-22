use crate::{
    characters::{HORIZONTAL_BAR, VERTICAL_BAR},
    cursor::cursor_position,
    draw::{Color, Draw, Point},
    util::Vec2,
};

pub struct CursorGuide {
    points: Vec<Vec2<i32>>,
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
            origin: Vec2 { x, y },
            character,
            foreground: Color::Guide,
            background: Color::EmptyBackground,
        }
    }
}

impl Draw for CursorGuide {
    fn draw(&self) -> std::io::Result<Vec<Point<i32>>> {
        let c: Vec2<i32> = cursor_position().into();
        let mut points = vec![];
        self.points
            .iter()
            .for_each(|p| match (c.x == p.x, c.y == p.y) {
                (true, false) => {
                    for i in (p.y.min(c.y) + 1)..p.y.max(c.y) {
                        points.push(Self::get_point(p.x, i, VERTICAL_BAR));
                    }
                }
                (false, true) => {
                    for i in (p.x.min(c.y) + 1)..p.x.max(c.x) {
                        points.push(Self::get_point(i, p.y, HORIZONTAL_BAR));
                    }
                }
                _ => {}
            });
        Ok(points)
    }
}

/// Used for showing guides when the cursor lines up with an object in one dimension
pub trait GuidePoint {
    fn get_intersection_points(&self) -> Vec<Vec2<i32>>;
}
