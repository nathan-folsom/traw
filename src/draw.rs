use std::{io::Result, ops::Deref};

use crate::{cursor::cursor_position, mode::Anchor, util::Vec2};

/// Used for rendering an object at a specific location on the canvas
pub trait Draw {
    fn draw(&self) -> std::io::Result<Vec<Point<i32>>>;
}

/// Used for rendering an object at a specific location on the screen
pub trait DrawSticky {
    fn draw(&self) -> std::io::Result<Vec<Point<u16>>>;
}

/// Used for changing the color of rendered content
pub trait DrawOverlay {
    fn draw_overlay(&self) -> Result<(Vec<OverlayPoint>, Option<Color>, Option<Color>)>;
}

/// Used for determining how the cursor aligns with an object
pub trait CursorIntersect {
    fn get_intersection(&self, point: &Vec2<i32>) -> Intersection;
    fn get_cursor_intersection(&self) -> std::io::Result<Intersection> {
        let position = cursor_position();
        Ok(self.get_intersection(&position.into()))
    }
    fn hovered(&self) -> std::io::Result<bool> {
        Ok(!matches!(
            self.get_cursor_intersection()?,
            Intersection::None
        ))
    }
}

pub struct OverlayPoint {
    pub x: i32,
    pub y: i32,
}

pub enum Intersection {
    None,
    Edge(EdgeIntersection),
    Inner,
}

pub enum EdgeIntersection {
    /// If intersecting a rectangle, which corner is intersected
    Corner(Option<Anchor>),
    Side,
}

#[derive(Debug, Clone)]
pub struct Point<T> {
    pub origin: Vec2<T>,
    pub character: char,
    pub foreground: Color,
    pub background: Color,
}

impl<T> Deref for Point<T> {
    type Target = Vec2<T>;
    fn deref(&self) -> &Self::Target {
        &self.origin
    }
}

impl From<Point<u16>> for Point<i32> {
    fn from(val: Point<u16>) -> Self {
        Point {
            origin: val.origin.into(),
            character: val.character,
            foreground: val.foreground,
            background: val.background,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Color {
    Empty,
    EmptyBackground,
    Border,
    BorderBackground,
    BorderBackgroundHover,
    Debug,
    DebugBackground,
    Grid,
    Guide,
}

impl From<Color> for crossterm::style::Color {
    fn from(value: Color) -> Self {
        let (r, g, b) = match value {
            Color::Border => (255, 255, 255),
            Color::BorderBackground => (0, 0, 0),
            Color::BorderBackgroundHover => (70, 70, 70),
            Color::Debug => (240, 240, 240),
            Color::DebugBackground => (40, 40, 40),
            Color::Empty => (255, 255, 255),
            Color::EmptyBackground => (0, 0, 0),
            Color::Grid => (100, 100, 40),
            Color::Guide => (120, 20, 20),
        };
        Self::Rgb { r, g, b }
    }
}
