use std::io::{stdout, Write};

use crossterm::{
    queue,
    style::{Print, SetBackgroundColor, SetForegroundColor},
};

use crate::{
    cursor::{restore_position, save_position, set_position},
    draw::{Color, DrawOverlay, OverlayPoint, Point},
};

pub struct Renderer {
    pub state: Vec<Vec<Cell>>,
    prev_state: Vec<Vec<Cell>>,
    width: u16,
    height: u16,
    is_first_frame: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Cell {
    pub character: char,
    pub shape_id: Option<u32>,
    foreground: Color,
    background: Color,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            state: vec![],
            prev_state: vec![],
            width,
            height,
            is_first_frame: true,
        }
    }

    pub fn render_frame<F>(&mut self, mut cb: F) -> std::io::Result<()>
    where
        F: FnMut(&mut Self) -> std::io::Result<()>,
    {
        self.start_frame();
        cb(self)?;
        self.finish_frame()?;
        if self.is_first_frame {
            self.is_first_frame = false;
        }
        stdout().flush()?;
        Ok(())
    }

    pub fn start_frame(&mut self) {
        let mut empty = vec![];
        for _ in 0..self.width {
            let mut cols = vec![];
            for _ in 0..self.height {
                cols.push(Cell {
                    character: ' ',
                    foreground: Color::Empty,
                    background: Color::EmptyBackground,
                    shape_id: None,
                });
            }
            empty.push(cols.clone());
        }
        if self.prev_state.is_empty() {
            self.prev_state = empty.clone();
        } else {
            std::mem::swap(&mut self.prev_state, &mut self.state);
        }
        self.state = empty;
    }

    pub fn render(
        &mut self,
        points: Vec<Point<i32>>,
        shape_id: Option<u32>,
    ) -> std::io::Result<()> {
        for point in points {
            self.draw_at(point, shape_id)?;
        }
        Ok(())
    }

    pub fn render_sticky(&mut self, points: Vec<Point<u16>>) -> std::io::Result<()> {
        for point in points {
            self.draw_at(point.into(), None)?;
        }
        Ok(())
    }

    fn draw_at(&mut self, point: Point<i32>, shape_id: Option<u32>) -> std::io::Result<()> {
        self.state[point.x as usize][point.y as usize] = Cell {
            character: point.character,
            foreground: point.foreground,
            background: point.background,
            shape_id,
        };

        Ok(())
    }

    pub fn render_overlay(&mut self, overlay: &impl DrawOverlay) -> std::io::Result<()> {
        let (points, foreground, background) = overlay.draw_overlay()?;
        for OverlayPoint { x, y } in points {
            let point = &mut self.state[x as usize][y as usize];
            if let Some(fg) = foreground {
                point.foreground = fg;
            }
            if let Some(bg) = background {
                point.background = bg;
            }
        }
        Ok(())
    }

    pub fn finish_frame(&self) -> std::io::Result<()> {
        save_position();
        self.state
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, point)| {
                        let prev = &self.prev_state[x][y];
                        if point != prev || self.is_first_frame {
                            set_position((x as u16, y as u16).into());
                            queue!(
                                stdout(),
                                SetForegroundColor(point.foreground.into()),
                                SetBackgroundColor(point.background.into()),
                                Print(point.character)
                            )?;
                        }
                        std::io::Result::Ok(())
                    })
                    .collect::<std::io::Result<Vec<_>>>()
            })
            .collect::<std::io::Result<Vec<_>>>()?;
        restore_position();
        Ok(())
    }
}
