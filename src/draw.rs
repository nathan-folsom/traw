use std::io::stdout;

use cli_clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::{
    cursor, queue,
    style::{Print, SetBackgroundColor, SetForegroundColor},
};

use crate::mode::{Anchor, Selection};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Color {
    Empty,
    EmptyBackground,
    Border,
    BorderBackground,
    BorderBackgroundHover,
}

const DEFAULT_COLORS: [(Color, crossterm::style::Color); 3] = [
    (
        Color::Border,
        crossterm::style::Color::Rgb {
            r: 255,
            g: 255,
            b: 255,
        },
    ),
    (
        Color::BorderBackground,
        crossterm::style::Color::Rgb { r: 0, g: 0, b: 0 },
    ),
    (
        Color::BorderBackgroundHover,
        crossterm::style::Color::Rgb {
            r: 70,
            g: 70,
            b: 70,
        },
    ),
];

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub character: char,
    pub foreground: Color,
    pub background: Color,
}

impl From<Point<u16>> for Point<i32> {
    fn from(val: Point<u16>) -> Self {
        Point {
            x: val.x as i32,
            y: val.y as i32,
            character: val.character,
            foreground: val.foreground,
            background: val.background,
        }
    }
}

pub trait Draw {
    fn draw(&self, hover: bool) -> std::io::Result<Vec<Point<i32>>>;
    fn get_intersection(&self) -> std::io::Result<Intersection>;
    fn clear(&self) -> std::io::Result<Vec<(i32, i32)>>;
}

pub trait DrawSticky {
    fn draw(&self) -> std::io::Result<Vec<Point<u16>>>;
}

pub trait DrawOverlay {
    fn draw(&self) -> (Vec<OverlayPoint>, Option<Color>, Option<Color>);
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

pub struct Renderer {
    state: Vec<Vec<(char, Color, Color)>>,
    prev_state: Vec<Vec<(char, Color, Color)>>,
    width: u16,
    height: u16,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            state: vec![],
            prev_state: vec![],
            width,
            height,
        }
    }

    pub fn start_frame(&mut self) {
        let mut empty = vec![];
        for _ in 0..self.width {
            let mut cols = vec![];
            for _ in 0..self.height {
                cols.push((' ', Color::Empty, Color::EmptyBackground));
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

    pub fn finish_frame(&self) -> std::io::Result<()> {
        queue!(stdout(), cursor::SavePosition)?;
        self.state
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, (character, foreground, background))| {
                        let (prev_char, prev_foreground, prev_background) = self.prev_state[x][y];
                        if character != &prev_char
                            || foreground != &prev_foreground
                            || background != &prev_background
                        {
                            queue!(
                                stdout(),
                                cursor::MoveTo(x as u16, y as u16),
                                SetForegroundColor(get_default_color(foreground, true)),
                                SetBackgroundColor(get_default_color(background, false)),
                                Print(character)
                            )?;
                        }
                        std::io::Result::Ok(())
                    })
                    .collect::<std::io::Result<Vec<_>>>()
            })
            .collect::<std::io::Result<Vec<_>>>()?;
        queue!(stdout(), cursor::RestorePosition)?;
        Ok(())
    }

    pub fn render(&mut self, shape: &impl Draw) -> std::io::Result<()> {
        let hover = !matches!(shape.get_intersection()?, Intersection::None);
        let points = shape.draw(hover)?;
        for point in points {
            self.draw_at(point)?;
        }
        Ok(())
    }

    pub fn render_sticky(&mut self, shape: &impl DrawSticky) -> std::io::Result<()> {
        let points = shape.draw()?;
        for point in points {
            self.draw_at(point.into())?;
        }
        Ok(())
    }

    pub fn render_overlay(&mut self, overlay: &impl DrawOverlay) -> std::io::Result<()> {
        let (points, foreground, background) = overlay.draw();
        for OverlayPoint { x, y } in points {
            let (_, current_foreground, current_background) =
                &mut self.state[x as usize][y as usize];
            if let Some(fg) = foreground {
                *current_foreground = fg;
            }
            if let Some(bg) = background {
                *current_background = bg;
            }
        }
        Ok(())
    }

    fn draw_at(&mut self, point: Point<i32>) -> std::io::Result<()> {
        let Point {
            x,
            y,
            character,
            foreground,
            background,
        } = point;
        self.state[x as usize][y as usize] = (character, foreground, background);

        Ok(())
    }

    pub fn handle_yank(&self, selection: &Selection) {
        let mut ctx = ClipboardContext::new().unwrap();
        let mut content = vec![];
        for row in 0..selection.height {
            for col in 0..selection.width {
                let x = col + selection.x;
                let y = row + selection.y;
                let (character, _, _) = self.state[x as usize][y as usize];
                content.push(character);
            }
            content.push('\n');
        }
        ctx.set_contents(content.iter().collect()).unwrap();
    }
}

fn get_default_color(color: &Color, fg: bool) -> crossterm::style::Color {
    let default = DEFAULT_COLORS.iter().find(|(c, _)| c == color);

    if let Some((_, rgb)) = default {
        *rgb
    } else if fg {
        crossterm::style::Color::Rgb {
            r: 255,
            g: 255,
            b: 255,
        }
    } else {
        crossterm::style::Color::Rgb { r: 0, g: 0, b: 0 }
    }
}
