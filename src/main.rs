use std::io::{stdout, Write};

use arrow::Arrow;
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use draw::Intersection;
use mode::Mode;
use rectangle::Rectangle;
use state::State;

mod arrow;
mod characters;
mod draw;
mod mode;
mod rectangle;
mod state;
mod status_bar;

fn main() -> std::io::Result<()> {
    let _ = init()?;
    let mut state = State::init()?;

    loop {
        match event::read()? {
            event::Event::Key(key_event) => {
                match state.mode {
                    Mode::Normal => {
                        handle_motions(key_event)?;
                    }
                    Mode::Text(_) => {}
                    Mode::DrawRectangle(_) => {
                        handle_motions(key_event)?;
                    }
                    Mode::DrawArrow(_) => {
                        handle_motions(key_event)?;
                    }
                }

                match key_event.code {
                    KeyCode::Char(key) => match &mut state.mode {
                        Mode::Text(rect) => {
                            rect.on_char(key)?;
                        }
                        Mode::Normal | Mode::DrawRectangle(_) => match key {
                            'q' => break,
                            'i' => match state.mode {
                                Mode::DrawRectangle(rect) => {
                                    state.mode = get_text_mode(rect)?;
                                }
                                Mode::Normal => {
                                    let (x, y) = cursor::position()?;
                                    let (intersection, _) = state.get_cursor_intersection()?;

                                    match intersection {
                                        Intersection::None => {
                                            state.mode = Mode::DrawRectangle(Rectangle::new_at(
                                                x as i32, y as i32,
                                            ));
                                        }
                                        Intersection::Edge => {
                                            state.mode = Mode::DrawArrow(Arrow::init());
                                        }
                                        Intersection::Inner => {
                                            todo!();
                                            // state.mode = Mode::Text(());
                                        }
                                    }
                                }
                                _ => {}
                            },
                            'x' => match state.mode {
                                Mode::Normal => {
                                    let (intersection, i) = state.get_cursor_intersection()?;
                                    match intersection {
                                        Intersection::Edge | Intersection::Inner => {
                                            state.renderer.clear(&*state.shapes[i])?;
                                            state.shapes.remove(i);
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    },

                    KeyCode::Enter => match state.mode {
                        Mode::DrawRectangle(rect) => {
                            state.mode = get_text_mode(rect)?;
                        }
                        Mode::Text(rect) => {
                            state.shapes.push(Box::new(rect));
                            state.mode = Mode::Normal;
                        }
                        Mode::DrawArrow(arrow) => {
                            state.shapes.push(Box::new(arrow));
                            state.mode = Mode::Normal;
                        }
                        _ => {}
                    },
                    KeyCode::Backspace => match &mut state.mode {
                        Mode::Text(rect) => {
                            rect.on_backspace()?;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        }

        state.status_bar.update(&state.mode)?;
        state.renderer.render_sticky(&state.status_bar)?;
        for shape in &state.shapes {
            state.renderer.render(&**shape)?;
        }

        match &mut state.mode {
            Mode::Normal => {}
            Mode::DrawRectangle(rect) => {
                rect.update()?;
                state.renderer.render(rect)?;
            }
            Mode::Text(rect) => {
                state.renderer.render(rect)?;
            }
            Mode::DrawArrow(arrow) => {
                arrow.update(cursor::position()?);
                state.renderer.render(arrow)?;
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

fn handle_motions(event: KeyEvent) -> std::io::Result<()> {
    match event.code {
        KeyCode::Char(key) => match key {
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
    }

    Ok(())
}

fn get_text_mode(rect: Rectangle) -> std::io::Result<Mode> {
    queue!(
        stdout(),
        cursor::MoveTo(rect.x as u16 + 1, rect.y as u16 + 1)
    )?;

    Ok(Mode::Text(rect))
}
