use std::io::stdout;

use crossterm::{cursor, queue, style::Print};

use crate::draw::Draw;

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

impl Draw for Rectangle {
    fn draw(&self) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor::position()?;

        for y in 0..self.height {
            for x in 0..self.width {
                let is_first_row = y == 0;
                let is_last_row = y == self.height - 1;
                let is_first_col = x == 0;
                let is_last_col = x == self.width - 1;

                if is_first_row || is_last_row || is_first_col || is_last_col {
                    queue!(stdout(), cursor::MoveTo(self.x + x, self.y + y))?;

                    let mut border_char = '*';
                    unsafe {
                        if is_first_row && is_first_col {
                            border_char = char::from_u32_unchecked(0x0256d);
                        } else if is_first_row && is_last_col {
                            border_char = char::from_u32_unchecked(0x0256e);
                        } else if is_last_row && is_last_col {
                            border_char = char::from_u32_unchecked(0x0256f);
                        } else if is_last_row && is_first_col {
                            border_char = char::from_u32_unchecked(0x02570);
                        } else if is_first_row || is_last_row {
                            border_char = char::from_u32_unchecked(0x02500);
                        } else if is_first_col || is_last_col {
                            border_char = char::from_u32_unchecked(0x02502);
                        }
                    }

                    queue!(stdout(), Print(border_char))?;
                }
            }
        }

        queue!(stdout(), cursor::MoveTo(cursor_x, cursor_y))?;

        Ok(())
    }
}
