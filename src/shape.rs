use serde::{Deserialize, Serialize};

use crate::{
    components::{arrow::Arrow, rectangle::Rectangle},
    cursor_guide::GuidePoint,
    draw::{CursorIntersect, Draw, Point},
    util::Vec2,
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
    fn get_intersection(&self, p: &Vec2<i32>) -> crate::draw::Intersection {
        match self {
            Shape::Arrow(shape) => shape.get_intersection(p),
            Shape::Rectangle(shape) => shape.get_intersection(p),
        }
    }
}

impl GuidePoint for Shape {
    fn get_intersection_points(&self) -> Vec<Vec2<i32>> {
        match self {
            Shape::Rectangle(shape) => shape.get_intersection_points(),
            Shape::Arrow(shape) => shape.get_intersection_points(),
        }
    }
}
