use crossterm::terminal;

use crate::draw::{DrawSticky, Point};

const HEIGHT: u16 = 10;

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

        for y in 0..HEIGHT {
            if let Some(message) = unsafe { DEBUG.get_or_init(|| vec![]).get(y as usize) } {
                let mut i = 0;
                for c in message.chars() {
                    if i >= w {
                        break;
                    }

                    points.push(Point {
                        x: i,
                        y: h - 10 + y,
                        character: c,
                        foreground: crate::draw::Color::Empty,
                        background: crate::draw::Color::EmptyBackground,
                    });
                    i += 1;
                }
            }
        }

        Ok(points)
    }
}
