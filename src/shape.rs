use serde::{Deserialize, Serialize};

use crate::{
    arrow::Arrow,
    draw::{CursorGuide, CursorIntersect, Draw, Point},
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
    fn get_intersection(&self, x: &i32, y: &i32) -> crate::draw::Intersection {
        match self {
            Shape::Arrow(shape) => shape.get_intersection(x, y),
            Shape::Rectangle(shape) => shape.get_intersection(x, y),
        }
    }
}

impl CursorGuide for Shape {
    fn get_intersection_point(&self, x: &i32, y: &i32) -> Option<(i32, i32)> {
        match self {
            Shape::Rectangle(shape) => shape.get_intersection_point(x, y),
            _ => None,
        }
    }
}
