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
        for x in 0..rect.width {
            let is_first_row = y == 0;
            let is_last_row = y == rect.height - 1;
            let is_first_col = x == 0;
            let is_last_col = x == rect.width - 1;
            if is_first_row || is_last_row || is_first_col || is_last_col {
                queue!(stdout(), cursor::MoveTo(rect.x + x, rect.y + y))?;
                queue!(stdout(), Print("*"))?;
            }
        }
    }

    queue!(stdout(), cursor::MoveTo(cursor_x, cursor_y))?;

    Ok(())
}
