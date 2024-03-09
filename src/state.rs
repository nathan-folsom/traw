use std::io::stdout;

use crossterm::{
    cursor::{self},
    queue,
};
use serde::{Deserialize, Serialize};

use crate::{
    arrow::Arrow,
    debug_panel::debug,
    draw::{
        Draw,
        EdgeIntersection::{Corner, Side},
        Intersection,
    },
    mode::{Anchor, Mode, Selection},
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
            Mode::DrawRectangle(_, _) => {
                if let Mode::DrawRectangle(rect, _) = std::mem::take(&mut self.mode) {
                    self.enter_text_mode(rect)?;
                }
            }
            Mode::Normal => {
                let (x, y) = cursor::position()?;
                let (intersection, i) = self.get_cursor_intersection()?;

                match intersection {
                    Intersection::None => {
                        self.enter_mode(Mode::DrawRectangle(
                            Rectangle::new_at(x as i32, y as i32),
                            Anchor::BottomRight,
                        ));
                    }
                    Intersection::Edge(Side) => {
                        self.enter_mode(Mode::DrawArrow(Arrow::init()));
                    }
                    Intersection::Inner => {
                        let edited = self.shapes.remove(i);
                        match edited {
                            Shape::Box(rectangle) => {
                                self.enter_text_mode(rectangle)?;
                            }
                            Shape::Line(arrow) => {
                                self.enter_mode(Mode::DrawArrow(arrow));
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn handle_drag(&mut self) -> std::io::Result<()> {
        if let Mode::Normal = self.mode {
            let (intersection, i) = self.get_cursor_intersection()?;
            if let Intersection::Edge(Corner(Some(anchor))) = intersection {
                if let Shape::Box(rectangle) = self.shapes.remove(i) {
                    self.enter_mode(Mode::DrawRectangle(rectangle, anchor))
                }
            }
        }

        Ok(())
    }

    pub fn handle_select(&mut self) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor::position()?;
        self.enter_mode(Mode::Select(Selection {
            x: cursor_x as i32,
            y: cursor_y as i32,
            width: 1,
            height: 1,
        }));
        Ok(())
    }

    pub fn handle_enter(&mut self) -> std::io::Result<()> {
        match &self.mode {
            Mode::DrawRectangle(_, _) => {
                if let Mode::DrawRectangle(rect, _) = std::mem::take(&mut self.mode) {
                    // Only start editing text if this is a new rectangle
                    if rect.text.is_empty() {
                        self.enter_text_mode(rect)?;
                    } else if let Mode::DrawRectangle(rect, _) =
                        std::mem::replace(&mut self.mode, Mode::Normal)
                    {
                        self.shapes.push(Shape::Box(rect));
                        self.enter_mode(Mode::Normal);
                    }
                }
            }
            Mode::Text(_) => {
                if let Mode::Text(rect) = std::mem::replace(&mut self.mode, Mode::Normal) {
                    self.shapes.push(Shape::Box(rect));
                    queue!(stdout(), cursor::SetCursorStyle::SteadyBlock)?;
                    self.enter_mode(Mode::Normal);
                }
            }
            Mode::DrawArrow(arrow) => {
                self.shapes.push(Shape::Line(arrow.clone()));
                self.enter_mode(Mode::Normal);
            }
            Mode::Select(_) => {
                self.enter_mode(Mode::Normal);
            }
            _ => {}
        }

        Ok(())
    }

    fn enter_text_mode(&mut self, rect: Rectangle) -> std::io::Result<()> {
        queue!(stdout(), cursor::SetCursorStyle::SteadyBar)?;
        let (next_x, next_y) = rect.get_inner_cursor_position();
        self.enter_mode(Mode::Text(rect));
        queue!(stdout(), cursor::MoveTo(next_x as u16, next_y as u16))?;

        Ok(())
    }

    fn enter_mode(&mut self, mode: Mode) {
        debug(format!("Enter mode {:?}", mode));
        self.mode = mode;
    }

    pub fn handle_delete(&mut self) -> std::io::Result<()> {
        if let Mode::Normal = self.mode {
            let (intersection, i) = self.get_cursor_intersection()?;
            match intersection {
                Intersection::Edge(_) | Intersection::Inner => {
                    self.shapes.remove(i);
                }
                _ => {}
            }
        }

        Ok(())
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

    pub fn handle_backspace(&mut self) -> std::io::Result<()> {
        if let Mode::Text(rect) = &mut self.mode {
            rect.on_backspace()?;
        }
        Ok(())
    }
}
