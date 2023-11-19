use crate::{arrow::Arrow, draw::Draw, rectangle::Rectangle};

pub enum Shape {
    Box(Rectangle),
    Line(Arrow),
}

impl Draw for Shape {
    fn draw(&self) -> std::io::Result<Vec<(i32, i32, char)>> {
        match self {
            Shape::Line(shape) => shape.draw(),
            Shape::Box(shape) => shape.draw(),
        }
    }
    fn clear(&self) -> std::io::Result<Vec<(i32, i32)>> {
        match self {
            Shape::Line(shape) => shape.clear(),
            Shape::Box(shape) => shape.clear(),
        }
    }
    fn get_intersection(&self) -> std::io::Result<crate::draw::Intersection> {
        match self {
            Shape::Line(shape) => shape.get_intersection(),
            Shape::Box(shape) => shape.get_intersection(),
        }
    }
}
