use std::{
    cmp::Ordering,
    io::stdout,
    sync::{OnceLock, RwLock},
};

use crossterm::{cursor, queue};

use crate::util::Vec2;

/// Ideally crossterm would keep track of the cursor position for us, but it seems
/// like there are some unfortunate performance side effects if we try and call
/// crossterm::cursor::position, possibly because crossterm is flushing buffered commands every time
/// we call it https://github.com/crossterm-rs/crossterm/issues/459
///
/// So instead we track the cursor position locally and try to keep it in sync with where the
/// cursor is being moved in stdout

static CURSOR: OnceLock<RwLock<Cursor>> = OnceLock::new();

struct Cursor {
    position: Vec2<u16>,
    saved_position: Option<Vec2<u16>>,
}

pub fn cursor_position() -> Vec2<u16> {
    CURSOR.get_or_init(init).read().unwrap().position.clone()
}

pub fn adjust_position(Vec2 { x, y }: Vec2<i16>) {
    let mut cursor = CURSOR.get().unwrap().write().unwrap();
    match x.cmp(&0) {
        Ordering::Less => {
            let _ = queue!(stdout(), cursor::MoveLeft(x.unsigned_abs()));
            cursor.position.x -= x.unsigned_abs();
        }
        Ordering::Greater => {
            let _ = queue!(stdout(), cursor::MoveRight(x as u16));
            cursor.position.x += x as u16;
        }
        _ => {}
    }
    match y.cmp(&0) {
        Ordering::Less => {
            let _ = queue!(stdout(), cursor::MoveLeft(y.unsigned_abs()));
            cursor.position.y -= y.unsigned_abs();
        }
        Ordering::Greater => {
            let _ = queue!(stdout(), cursor::MoveRight(y as u16));
            cursor.position.y += y as u16;
        }
        _ => {}
    }
}

pub fn set_position(Vec2 { x, y }: Vec2<u16>) {
    let _ = queue!(stdout(), cursor::MoveTo(x, y));
    let mut cursor = CURSOR.get_or_init(init).write().unwrap();
    cursor.position.x = x;
    cursor.position.y = y;
}

pub fn save_position() {
    let _ = queue!(stdout(), cursor::SavePosition);
    let mut cursor = CURSOR.get_or_init(init).write().unwrap();
    cursor.saved_position = Some(cursor.position.clone());
}

pub fn restore_position() {
    let _ = queue!(stdout(), cursor::RestorePosition);
    let position = {
        let mut cursor = CURSOR.get_or_init(init).write().unwrap();
        let saved = cursor.saved_position.take();
        saved.unwrap()
    };
    set_position(position)
}

fn init() -> RwLock<Cursor> {
    let position = cursor::position().expect("Failed to init cursor position");
    RwLock::new(Cursor {
        position: position.into(),
        saved_position: None,
    })
}
