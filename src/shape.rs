use serde::{Deserialize, Serialize};

use crate::{
    arrow::Arrow,
    cursor_guide::GuidePoint,
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
    fn get_intersection(&self, x: &i32, y: &i32) -> crate::draw::Intersection {
        match self {
            Shape::Arrow(shape) => shape.get_intersection(x, y),
            Shape::Rectangle(shape) => shape.get_intersection(x, y),
        }
    }
}

impl GuidePoint for Shape {
    fn get_intersection_points(&self) -> Vec<(i32, i32)> {
        match self {
            Shape::Rectangle(shape) => shape.get_intersection_points(),
            _ => vec![],
        }
    }
}
