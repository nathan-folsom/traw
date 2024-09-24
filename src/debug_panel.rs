use crossterm::terminal;

use crate::draw::{DrawSticky, Point};

pub const DEBUG_PANEL_HEIGHT: usize = 20;

use std::{sync::OnceLock, time::Instant};
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

#[allow(dead_code)] // Util that is often used between commits
pub fn time_since(message: &str, last: &mut Instant) {
    let now = Instant::now();
    debug(format!("At {} {:?}", message, now - *last));
    *last = now;
}

impl DrawSticky for DebugPanel {
    fn draw(&self) -> std::io::Result<Vec<crate::draw::Point<u16>>> {
        let (w, h) = terminal::size()?;
        let mut points = vec![];

        let messages = unsafe { DEBUG.get_or_init(Vec::new) };
        for y in 0..DEBUG_PANEL_HEIGHT {
            let message_idx = messages.len().checked_sub(DEBUG_PANEL_HEIGHT - y);
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
                    y: h - DEBUG_PANEL_HEIGHT as u16 + y as u16,
                    character,
                    foreground: crate::draw::Color::Debug,
                    background: crate::draw::Color::DebugBackground,
                });
            }
        }

        Ok(points)
    }
}
