use std::io::stdout;

use crossterm::{
    cursor, queue,
    style::{Print, SetBackgroundColor, SetForegroundColor},
};

#[derive(PartialEq, Eq, Copy, Clone)]
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
        crossterm::style::Color::Rgb { r: 0, g: 255, b: 0 },
    ),
    (
        Color::BorderBackground,
        crossterm::style::Color::Rgb { r: 0, g: 0, b: 0 },
    ),
    (
        Color::BorderBackgroundHover,
        crossterm::style::Color::Rgb {
            r: 200,
            g: 200,
            b: 200,
        },
    ),
];

pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub character: char,
    pub foreground: Color,
    pub background: Color,
}

impl Into<Point<i32>> for Point<u16> {
    fn into(self) -> Point<i32> {
        Point {
            x: self.x as i32,
            y: self.y as i32,
            character: self.character,
            foreground: self.foreground,
            background: self.background,
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

pub enum Intersection {
    None,
    Edge,
    Inner,
}

pub struct Renderer {
    // x: u32,
    // y: u32,
    // width: u16,
    // height: u16,
    state: Vec<Vec<char>>,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        let mut initial_state = vec![];
        for _ in 0..width {
            let mut cols = vec![];
            for _ in 0..height {
                cols.push(' ');
            }
            initial_state.push(cols);
        }

        Self {
            //         x: 0,
            //         y: 0,
            //         width,
            //         height,
            state: initial_state,
        }
    }

    pub fn render(&mut self, shape: &impl Draw) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor::position()?;
        let hover = match shape.get_intersection()? {
            Intersection::None => false,
            _ => true,
        };
        let points = shape.draw(hover)?;

        for point in points {
            self.draw_at(point)?;
        }

        queue!(stdout(), cursor::MoveTo(cursor_x, cursor_y),)?;

        Ok(())
    }

    pub fn render_sticky(&mut self, shape: &impl DrawSticky) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor::position()?;
        let points = shape.draw()?;

        for point in points {
            self.draw_at(point.into())?;
        }

        queue!(stdout(), cursor::MoveTo(cursor_x, cursor_y),)?;

        Ok(())
    }

    pub fn clear(&mut self, shape: &impl Draw) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor::position()?;
        let points = shape.clear()?;

        for (x, y) in points {
            self.draw_at(Point {
                x,
                y,
                character: ' ',
                foreground: Color::Empty,
                background: Color::EmptyBackground,
            })?;
        }

        queue!(stdout(), cursor::MoveTo(cursor_x, cursor_y),)?;

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
        let current_char = self.state[x as usize][y as usize];

        if current_char != character {
            queue!(
                stdout(),
                cursor::MoveTo(x as u16, y as u16),
                SetForegroundColor(get_default_color(foreground, true)),
                SetBackgroundColor(get_default_color(background, false)),
                Print(character)
            )?;
            self.state[x as usize][y as usize] = character;
        }

        Ok(())
    }
}

fn get_default_color(color: Color, fg: bool) -> crossterm::style::Color {
    let default = DEFAULT_COLORS.iter().find(|(c, _)| c == &color);

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
