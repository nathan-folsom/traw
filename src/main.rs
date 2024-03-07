use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use debug_panel::DebugPanel;
use draw::{Intersection, Renderer};
use mode::Mode;
use persistence::{load, save};
use rectangle::Rectangle;
use shape::Shape;
use state::State;
use status_bar::StatusBar;

mod arrow;
mod characters;
mod debug_panel;
mod draw;
mod mode;
mod persistence;
mod rectangle;
mod shape;
mod state;
mod status_bar;

fn main() -> std::io::Result<()> {
    let _ = init()?;
    let mut state = State::init();
    let (width, height) = terminal::size()?;
    let mut renderer = Renderer::new(width, height);
    let mut status_bar = StatusBar::default();
    let debug_panel = DebugPanel::default();
    let path_arg = std::env::args().nth(1);

    let mut file_name = "unnamed.traw".to_string();

    if let Some(path) = path_arg {
        file_name = path;
        state = load(&file_name)?;
        status_bar.update(&state.mode)?;
        renderer.render_sticky(&status_bar)?;
        for shape in &state.shapes {
            renderer.render(shape)?;
        }
        stdout().flush()?;
    }

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
                            's' => save(&state, &file_name)?,
                            'i' => state.handle_insert()?,
                            'x' => match state.mode {
                                Mode::Normal => {
                                    let (intersection, i) = state.get_cursor_intersection()?;
                                    match intersection {
                                        Intersection::Edge | Intersection::Inner => {
                                            renderer.clear(&state.shapes[i])?;
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
                            state.shapes.push(Shape::Box(rect));
                            state.mode = Mode::Normal;
                        }
                        Mode::DrawArrow(arrow) => {
                            state.shapes.push(Shape::Line(arrow));
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

        status_bar.update(&state.mode)?;
        renderer.render_sticky(&status_bar)?;
        renderer.render_sticky(&debug_panel)?;
        for shape in &state.shapes {
            renderer.render(shape)?;
        }

        match &mut state.mode {
            Mode::Normal => {}
            Mode::DrawRectangle(rect) => {
                rect.update()?;
                renderer.render(rect)?;
            }
            Mode::Text(rect) => {
                renderer.render(rect)?;
            }
            Mode::DrawArrow(arrow) => {
                arrow.update(cursor::position()?);
                renderer.render(arrow)?;
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
    let (next_x, next_y) = rect.get_inner_cursor_position();
    queue!(stdout(), cursor::MoveTo(next_x as u16, next_y as u16))?;

    Ok(Mode::Text(rect))
}
