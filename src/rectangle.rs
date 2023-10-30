use std::io::stdout;

use crossterm::{cursor, queue, style::Print};

#[derive(Default, Debug)]
pub struct Rectangle {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rectangle {
    pub fn new_at(x: u16, y: u16) -> Rectangle {
        Self {
            x,
            y,
            width: 1,
            height: 1,
        }
    }
}

pub fn draw_rectangle(rect: &Rectangle) -> std::io::Result<()> {
    let (cursor_x, cursor_y) = cursor::position()?;

    for y in 0..rect.height {
        queue!(stdout(), cursor::MoveTo(rect.x, rect.y + y))?;
        for _ in 0..rect.width {
            queue!(stdout(), Print("*"))?;
        }
    }

    queue!(stdout(), cursor::MoveTo(cursor_x, cursor_y))?;

    Ok(())
}
