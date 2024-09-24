use std::sync::OnceLock;

use crossterm::cursor;

static mut CURSOR_POS: OnceLock<(u16, u16)> = OnceLock::new();

pub fn cursor_pos() -> (u16, u16) {
    unsafe { *CURSOR_POS.get().expect("Cursor position uninitialized") }
}

pub fn start_frame() {
    unsafe {
        let (next_x, next_y) = cursor::position().expect("Failed to set cursor position");
        if let Some((prev_x, prev_y)) = CURSOR_POS.get_mut() {
            *prev_x = next_x;
            *prev_y = next_y;
        } else {
            CURSOR_POS.get_or_init(|| (next_x, next_y));
        }
    }
}
