use std::io::{stdout, Result};

use crossterm::{
    cursor::{self},
    queue,
};
use serde::{Deserialize, Serialize};

use crate::{
    arrow::Arrow,
    cursor::{cursor_pos, set_position},
    cursor_guide::CursorGuide,
    debug_panel::debug,
    draw::{
        Color, CursorIntersect, Draw, DrawOverlay,
        EdgeIntersection::{Corner, Side},
        Intersection, OverlayPoint, Point,
    },
    mode::{Anchor, Mode, Selection},
    mutate::Mutate,
    rectangle::Rectangle,
    shape::Shape,
};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub shapes: Vec<Shape>,
    pub mode: Mode,
    pub debug_enabled: bool,
    undo_stack: Vec<StateChange>,
    redo_stack: Vec<StateChange>,
}

impl State {
    pub fn init() -> Self {
        Self {
            shapes: vec![],
            mode: Mode::Normal,
            debug_enabled: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn handle_insert(&mut self) -> std::io::Result<()> {
        if let Mode::Normal = &self.mode {
            let (x, y) = cursor_pos();
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
                        Shape::Rectangle(rectangle) => {
                            self.enter_text_mode(rectangle)?;
                        }
                        Shape::Arrow(arrow) => {
                            self.enter_mode(Mode::DrawArrow(arrow));
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn handle_drag(&mut self) -> std::io::Result<()> {
        if let Mode::Normal = self.mode {
            let (intersection, i) = self.get_cursor_intersection()?;
            if let Intersection::Edge(Corner(Some(anchor))) = intersection {
                if let Shape::Rectangle(rectangle) = self.shapes.remove(i) {
                    self.enter_mode(Mode::DrawRectangle(rectangle, anchor))
                }
            }
        }

        Ok(())
    }

    pub fn handle_select(&mut self) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor_pos();
        self.enter_mode(Mode::Select(Selection {
            x: cursor_x as i32,
            y: cursor_y as i32,
            width: 1,
            height: 1,
        }));
        Ok(())
    }

    pub fn handle_enter(&mut self) -> std::io::Result<()> {
        match std::mem::take(&mut self.mode) {
            Mode::DrawRectangle(rect, _) => {
                // Only start editing text if this is a new rectangle
                if rect.text.is_empty() {
                    self.enter_text_mode(rect)?;
                } else {
                    self.add_shape(Shape::Rectangle(rect));
                    self.enter_mode(Mode::Normal);
                }
            }
            Mode::Text(rect) => {
                self.add_shape(Shape::Rectangle(rect));
                queue!(stdout(), cursor::SetCursorStyle::SteadyBlock)?;
            }
            Mode::DrawArrow(arrow) => {
                self.add_shape(Shape::Arrow(arrow));
            }
            Mode::Select(_) => {
                self.enter_mode(Mode::Normal);
            }
            Mode::Normal => {}
        }

        Ok(())
    }

    fn add_shape(&mut self, shape: Shape) {
        let mx = self.mutate(StateChange::AddShape(shape));
        self.undo_stack.push(mx);
    }

    fn enter_text_mode(&mut self, rect: Rectangle) -> std::io::Result<()> {
        queue!(stdout(), cursor::SetCursorStyle::SteadyBar)?;
        let (next_x, next_y) = rect.get_inner_cursor_position();
        self.enter_mode(Mode::Text(rect));
        set_position(next_x as u16, next_y as u16);

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
                    let mx = self.mutate(StateChange::DeleteShape(i));
                    self.undo_stack.push(mx);
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn get_cursor_intersection(&self) -> std::io::Result<(Intersection, usize)> {
        for i in 0..self.shapes.len() {
            let shape = &self.shapes[i];
            match shape.get_cursor_intersection() {
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

    pub fn undo(&mut self) {
        if let Some(undo) = self.undo_stack.pop() {
            let redo = self.mutate(undo);
            self.redo_stack.push(redo);
        }
    }

    pub fn redo(&mut self) {
        if let Some(redo) = self.redo_stack.pop() {
            let undo = self.mutate(redo);
            self.undo_stack.push(undo);
        }
    }
}

impl Draw for State {
    fn draw(&self) -> std::io::Result<Vec<Point<i32>>> {
        let mut points = vec![CursorGuide::new(&self.shapes).draw()?];
        for shape in &self.shapes {
            points.push(shape.draw()?);
        }
        Ok(points.into_iter().flatten().collect())
    }
}

impl DrawOverlay for State {
    fn draw_overlay(
        &self,
    ) -> Result<(
        Vec<crate::draw::OverlayPoint>,
        Option<crate::draw::Color>,
        Option<crate::draw::Color>,
    )> {
        let mut overlay_points: Vec<Vec<OverlayPoint>> = vec![];
        for shape in &self.shapes {
            if shape.hovered()? {
                overlay_points.push(
                    shape
                        .draw()?
                        .into_iter()
                        .map(|p| OverlayPoint { x: p.x, y: p.y })
                        .collect(),
                )
            }
        }
        Ok((
            overlay_points.into_iter().flatten().collect(),
            None,
            Some(Color::BorderBackgroundHover),
        ))
    }
}

impl Mutate for State {
    type Mutation = StateChange;
    fn mutate(&mut self, mx: Self::Mutation) -> Self::Mutation {
        match mx {
            StateChange::DeleteShape(index) => {
                let removed = self.shapes.remove(index);
                StateChange::AddShape(removed)
            }
            StateChange::AddShape(shape) => {
                self.shapes.push(shape);
                let index = self.shapes.len() - 1;
                StateChange::DeleteShape(index)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum StateChange {
    DeleteShape(usize),
    AddShape(Shape),
}
