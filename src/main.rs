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
    let mut renderer = Renderer::new(width, height);

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
                            new_rectangle = Some(Rectangle::new_at(x as i32, y as i32));
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
            rect.update()?;
            renderer.render(rect)?;
        }

        for r in &rectangles {
            renderer.render(r)?;
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
