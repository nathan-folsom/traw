use std::{
    cmp::Ordering,
    io::stdout,
    sync::{OnceLock, RwLock},
};

use crossterm::{cursor, queue};

static CURSOR: OnceLock<RwLock<Cursor>> = OnceLock::new();

struct Cursor {
    x: u16,
    y: u16,
    saved_position: Option<(u16, u16)>,
}

impl Cursor {
    fn position(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

pub fn cursor_pos() -> (u16, u16) {
    CURSOR.get_or_init(init).read().unwrap().position()
}

pub fn adjust_position(x: i16, y: i16) {
    let mut cursor = CURSOR.get().unwrap().write().unwrap();
    match x.cmp(&0) {
        Ordering::Less => {
            let _ = queue!(stdout(), cursor::MoveLeft(x.unsigned_abs()));
            cursor.x -= x.unsigned_abs();
        }
        Ordering::Greater => {
            let _ = queue!(stdout(), cursor::MoveRight(x as u16));
            cursor.x += x as u16;
        }
        _ => {}
    }
    match y.cmp(&0) {
        Ordering::Less => {
            let _ = queue!(stdout(), cursor::MoveLeft(y.unsigned_abs()));
            cursor.y -= y.unsigned_abs();
        }
        Ordering::Greater => {
            let _ = queue!(stdout(), cursor::MoveRight(y as u16));
            cursor.y += y as u16;
        }
        _ => {}
    }
}

pub fn set_position(x: u16, y: u16) {
    let _ = queue!(stdout(), cursor::MoveTo(x, y));
    let mut cursor = CURSOR.get_or_init(init).write().unwrap();
    cursor.x = x;
    cursor.y = y;
}

pub fn save_position() {
    let _ = queue!(stdout(), cursor::SavePosition);
    let mut cursor = CURSOR.get_or_init(init).write().unwrap();
    cursor.saved_position = Some(cursor.position());
}

pub fn restore_position() {
    let _ = queue!(stdout(), cursor::RestorePosition);
    let (x, y) = {
        let mut cursor = CURSOR.get_or_init(init).write().unwrap();
        let saved = cursor.saved_position.unwrap();
        cursor.saved_position = None;
        saved
    };
    set_position(x, y)
}

fn init() -> RwLock<Cursor> {
    let (x, y) = cursor::position().expect("Failed to init cursor position");
    RwLock::new(Cursor {
        x,
        y,
        saved_position: None,
    })
}
