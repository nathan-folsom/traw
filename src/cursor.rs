use std::{
    cmp::Ordering,
    io::stdout,
    sync::{OnceLock, RwLock},
};

use crossterm::{cursor, queue};

static CURSOR: OnceLock<RwLock<(u16, u16)>> = OnceLock::new();

pub fn cursor_pos() -> (u16, u16) {
    *CURSOR.get_or_init(init).read().unwrap()
}

pub fn adjust_position(x: i16, y: i16) {
    let mut position = CURSOR.get().unwrap().write().unwrap();
    match x.cmp(&0) {
        Ordering::Less => {
            let _ = queue!(stdout(), cursor::MoveLeft(x.unsigned_abs()));
            position.0 -= x.unsigned_abs();
        }
        Ordering::Greater => {
            let _ = queue!(stdout(), cursor::MoveRight(x as u16));
            position.0 += x as u16;
        }
        _ => {}
    }
    match y.cmp(&0) {
        Ordering::Less => {
            let _ = queue!(stdout(), cursor::MoveLeft(y.unsigned_abs()));
            position.1 -= y.unsigned_abs();
        }
        Ordering::Greater => {
            let _ = queue!(stdout(), cursor::MoveRight(y as u16));
            position.1 += y as u16;
        }
        _ => {}
    }
}

pub fn set_position(x: u16, y: u16) {
    let _ = queue!(stdout(), cursor::MoveTo(x, y));
    *CURSOR.get_or_init(init).write().unwrap() = (x, y);
}

fn init() -> RwLock<(u16, u16)> {
    RwLock::new(cursor::position().expect("Failed to init cursor position"))
}
