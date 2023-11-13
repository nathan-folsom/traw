use crossterm::terminal;

use crate::{
    arrow::Arrow,
    draw::Renderer,
    mode::Mode,
    rectangle::{Rectangle, RectangleIntersection},
    status_bar::StatusBar,
};

pub struct State {
    pub rectangles: Vec<Rectangle>,
    pub arrows: Vec<Arrow>,
    pub renderer: Renderer,
    pub mode: Mode,
    pub status_bar: StatusBar,
}

impl State {
    pub fn init() -> std::io::Result<Self> {
        let (width, height) = terminal::size()?;

        Ok(Self {
            rectangles: vec![],
            arrows: vec![],
            renderer: Renderer::new(width, height),
            mode: Mode::Normal,
            status_bar: StatusBar::default(),
        })
    }

    pub fn get_rectangle_intersection(&self) -> std::io::Result<RectangleIntersection> {
        for rectangle in &self.rectangles {
            match rectangle.get_intersection() {
                Ok(RectangleIntersection::None) => {}
                Ok(RectangleIntersection::Inner | RectangleIntersection::Edge) => {
                    return rectangle.get_intersection();
                }
                _ => {}
            }
        }

        Ok(RectangleIntersection::None)
    }
}
