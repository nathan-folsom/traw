use serde::{Deserialize, Serialize};

use crate::{
    arrow::Arrow,
    draw::{CursorIntersect, Draw, Point},
    rectangle::Rectangle,
};

#[derive(Serialize, Deserialize)]
pub enum Shape {
    Box(Rectangle),
    Line(Arrow),
}

impl Draw for Shape {
    fn draw(&self) -> std::io::Result<Vec<Point<i32>>> {
        match self {
            Shape::Line(shape) => shape.draw(),
            Shape::Box(shape) => shape.draw(),
        }
    }
}

impl CursorIntersect for Shape {
    fn get_intersection(&self) -> std::io::Result<crate::draw::Intersection> {
        match self {
            Shape::Line(shape) => shape.get_intersection(),
            Shape::Box(shape) => shape.get_intersection(),
        }
    }
}
