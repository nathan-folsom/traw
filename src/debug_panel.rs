use crossterm::terminal;

use crate::draw::{DrawSticky, Point};

const HEIGHT: usize = 10;

use std::sync::OnceLock;
static mut DEBUG: OnceLock<Vec<String>> = OnceLock::new();

#[derive(Default)]
pub struct DebugPanel {}

pub fn debug(message: String) {
    unsafe {
        let _ = DEBUG.set(vec![]);
        DEBUG
            .get_mut()
            .expect("DEBUG messages uninitialized")
            .push(message);
    }
}

impl DrawSticky for DebugPanel {
    fn draw(&self) -> std::io::Result<Vec<crate::draw::Point<u16>>> {
        let (w, h) = terminal::size()?;
        let mut points = vec![];

        let messages = unsafe { DEBUG.get_or_init(|| vec![]) };
        for y in 0..HEIGHT {
            let end_offset = HEIGHT - y;
            if end_offset > messages.len() {
                continue;
            }
            if let Some(message) = messages.get(messages.len() - end_offset) {
                for x in 0..w {
                    let character = match message.chars().nth(x as usize) {
                        Some(c) => c,
                        None => ' ',
                    };
                    points.push(Point {
                        x,
                        y: h - 10 + y as u16,
                        character,
                        foreground: crate::draw::Color::Empty,
                        background: crate::draw::Color::EmptyBackground,
                    });
                }
            }
        }

        Ok(points)
    }
}
