use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use draw::Renderer;
use rectangle::Rectangle;

mod draw;
mod rectangle;

fn main() -> std::io::Result<()> {
    let _ = init()?;
    let mut rectangles = vec![];
    let mut new_rectangle = None;
    let (width, height) = terminal::size()?;
    let renderer = Renderer::new(width, height);

    loop {
        match event::read()? {
            event::Event::Key(key_event) => match key_event.code {
                KeyCode::Char(key) => match key {
                    'q' => break,
                    'r' => {
                        if let Some(rect) = new_rectangle {
                            new_rectangle = None;
                            rectangles.push(rect);
                        } else {
                            let (x, y) = cursor::position()?;
                            new_rectangle = Some(Rectangle::new_at(x, y));
                        }
                    }
                    'h' => {
                        execute!(stdout(), cursor::MoveLeft(1))?;
                    }
                    'j' => {
                        execute!(stdout(), cursor::MoveDown(1))?;
                    }
                    'k' => {
                        execute!(stdout(), cursor::MoveUp(1))?;
                    }
                    'l' => {
                        execute!(stdout(), cursor::MoveRight(1))?;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        if let Some(rect) = &mut new_rectangle {
            let (cursor_x, cursor_y) = cursor::position()?;
            if cursor_x >= rect.x && cursor_y >= rect.y {
                rect.width = cursor_x - rect.x + 1;
                rect.height = cursor_y - rect.y + 1;
            }

            renderer.render(rect)?;
        }

        stdout().flush()?;
    }

    end()?;

    Ok(())
}

fn init() -> std::io::Result<()> {
    enable_raw_mode()?;

    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
        SetForegroundColor(Color::White),
        ResetColor
    )?;

    Ok(())
}

fn end() -> std::io::Result<()> {
    execute!(stdout(), terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
