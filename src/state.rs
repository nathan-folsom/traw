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

    pub fn get_rectangle_intersection(&self) -> std::io::Result<(RectangleIntersection, usize)> {
        for i in 0..self.rectangles.len() {
            let rectangle = &self.rectangles[i];
            let intersection = rectangle.get_intersection();
            match rectangle.get_intersection() {
                Ok(RectangleIntersection::None) => {}
                Ok(intersection_type) => {
                    return Ok((intersection_type, i));
                }
                _ => {}
            }
        }

        Ok((RectangleIntersection::None, 0))
    }
}
