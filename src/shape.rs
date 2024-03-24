use serde::{Deserialize, Serialize};

use crate::{
    arrow::Arrow,
    draw::{CursorIntersect, Draw, Point},
    rectangle::Rectangle,
};

#[derive(Serialize, Deserialize)]
pub enum Shape {
    Rectangle(Rectangle),
    Arrow(Arrow),
}

impl Draw for Shape {
    fn draw(&self) -> std::io::Result<Vec<Point<i32>>> {
        match self {
            Shape::Arrow(shape) => shape.draw(),
            Shape::Rectangle(shape) => shape.draw(),
        }
    }
}

impl CursorIntersect for Shape {
    fn get_intersection(&self) -> std::io::Result<crate::draw::Intersection> {
        match self {
            Shape::Arrow(shape) => shape.get_intersection(),
            Shape::Rectangle(shape) => shape.get_intersection(),
        }
    }
}
