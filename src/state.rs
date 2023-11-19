use crossterm::terminal;

use crate::{
    arrow::Arrow,
    draw::{Draw, Intersection, Renderer},
    mode::Mode,
    rectangle::Rectangle,
    status_bar::StatusBar,
};

pub struct State {
    pub rectangles: Vec<Rectangle>,
    pub arrows: Vec<Arrow>,
    pub shapes: Vec<Box<dyn Draw>>,
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
            shapes: vec![],
            renderer: Renderer::new(width, height),
            mode: Mode::Normal,
            status_bar: StatusBar::default(),
        })
    }

    pub fn get_cursor_intersection(&self) -> std::io::Result<(Intersection, usize)> {
        for i in 0..self.shapes.len() {
            let shape = &self.rectangles[i];
            match shape.get_intersection() {
                Ok(Intersection::None) => {}
                Ok(intersection_type) => {
                    return Ok((intersection_type, i));
                }
                _ => {}
            }
        }

        Ok((Intersection::None, 0))
    }
}
