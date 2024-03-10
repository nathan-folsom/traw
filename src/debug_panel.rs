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

        let messages = unsafe { DEBUG.get_or_init(Vec::new) };
        for y in 0..HEIGHT {
            let message_idx = messages.len().checked_sub(HEIGHT - y);
            let message = match message_idx {
                Some(i) => {
                    if let Some(m) = messages.get(i) {
                        m.clone()
                    } else {
                        "".to_string()
                    }
                }
                None => "".to_string(),
            };
            for x in 0..w {
                let character = message.chars().nth(x as usize).unwrap_or(' ');
                points.push(Point {
                    x,
                    y: h - 10 + y as u16,
                    character,
                    foreground: crate::draw::Color::Debug,
                    background: crate::draw::Color::DebugBackground,
                });
            }
        }

        Ok(points)
    }
}
