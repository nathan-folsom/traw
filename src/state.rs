use crossterm::terminal;

use crate::{
    arrow::Arrow, draw::Renderer, mode::Mode, rectangle::Rectangle, status_bar::StatusBar,
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
}
