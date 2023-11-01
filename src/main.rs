use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute, queue,
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
    let (width, height) = terminal::size()?;
    let mut renderer = Renderer::new(width, height);
    let mut mode = Mode::Normal;

    loop {
        match event::read()? {
            event::Event::Key(key_event) => match key_event.code {
                KeyCode::Char(key) => match &mut mode {
                    Mode::Text(rect) => {
                        rect.on_char(key);
                        queue!(stdout(), cursor::MoveRight(1))?;
                    }
                    Mode::Normal | Mode::Draw(_) => match key {
                        'q' => break,
                        'i' => match mode {
                            Mode::Draw(rect) => {
                                mode = Mode::Text(rect);
                            }
                            Mode::Normal => {
                                let (x, y) = cursor::position()?;
                                mode = Mode::Draw(Rectangle::new_at(x as i32, y as i32));
                            }
                            _ => {}
                        },
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
                },
                KeyCode::Enter => match mode {
                    Mode::Draw(rect) => {
                        queue!(
                            stdout(),
                            cursor::MoveTo(rect.x as u16 + 1, rect.y as u16 + 1)
                        )?;
                        mode = Mode::Text(rect);
                    }
                    Mode::Text(rect) => {
                        rectangles.push(rect);
                        mode = Mode::Normal;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }

        match &mut mode {
            Mode::Normal => {}
            Mode::Draw(rect) => {
                rect.update()?;
                renderer.render(rect)?;
            }
            Mode::Text(rect) => {
                renderer.render(rect)?;
            }
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

enum Mode {
    Normal,
    Draw(Rectangle),
    Text(Rectangle),
}
