use std::io::stdout;

use crossterm::{cursor, queue};
use serde::{Deserialize, Serialize};

use crate::{
    arrow::Arrow,
    debug_panel::debug,
    draw::{Draw, Intersection},
    mode::Mode,
    rectangle::Rectangle,
    shape::Shape,
};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub shapes: Vec<Shape>,
    pub mode: Mode,
}

impl State {
    pub fn init() -> Self {
        Self {
            shapes: vec![],
            mode: Mode::Normal,
        }
    }

    pub fn handle_insert(&mut self) -> std::io::Result<()> {
        match &self.mode {
            Mode::DrawRectangle(rect) => {
                self.enter_mode(get_text_mode(rect.clone())?);
            }
            Mode::Normal => {
                let (x, y) = cursor::position()?;
                let (intersection, i) = self.get_cursor_intersection()?;

                match intersection {
                    Intersection::None => {
                        self.enter_mode(Mode::DrawRectangle(Rectangle::new_at(x as i32, y as i32)));
                    }
                    Intersection::Edge => {
                        self.enter_mode(Mode::DrawArrow(Arrow::init()));
                    }
                    Intersection::Inner => {
                        let edited = self.shapes.remove(i);
                        match edited {
                            Shape::Box(rectangle) => {
                                self.enter_mode(get_text_mode(rectangle)?);
                            }
                            Shape::Line(arrow) => {
                                self.enter_mode(Mode::DrawArrow(arrow));
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn enter_mode(&mut self, mode: Mode) {
        debug(format!("Enter mode {:?}", mode));
        self.mode = mode;
    }

    pub fn get_cursor_intersection(&self) -> std::io::Result<(Intersection, usize)> {
        for i in 0..self.shapes.len() {
            let shape = &self.shapes[i];
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

fn get_text_mode(rect: Rectangle) -> std::io::Result<Mode> {
    let (next_x, next_y) = rect.get_inner_cursor_position();
    queue!(stdout(), cursor::MoveTo(next_x as u16, next_y as u16))?;

    Ok(Mode::Text(rect))
}
