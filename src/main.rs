use std::io::{stdout, Write};

use arrow::Arrow;
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use draw::{Intersection, Renderer};
use mode::Mode;
use rectangle::Rectangle;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use shape::Shape;
use state::State;
use status_bar::StatusBar;

mod arrow;
mod characters;
mod draw;
mod mode;
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
                            's' => save(&state)?,
                            'i' => match state.mode {
                                Mode::DrawRectangle(rect) => {
                                    state.mode = get_text_mode(rect)?;
                                }
                                Mode::Normal => {
                                    let (x, y) = cursor::position()?;
                                    let (intersection, i) = state.get_cursor_intersection()?;

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
                                            let edited = state.shapes.remove(i);
                                            match edited {
                                                Shape::Box(rectangle) => {
                                                    state.mode = get_text_mode(rectangle)?;
                                                }
                                                Shape::Line(arrow) => {
                                                    state.mode = Mode::DrawArrow(arrow);
                                                }
                                            }
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

#[derive(Serialize, Deserialize)]
struct TrawFile {
    version: FileVersion,
    data: String,
}

const CURRENT_VERSION: FileVersion = FileVersion::V1;

impl TrawFile {
    pub fn new(data: String) -> Self {
        TrawFile {
            version: CURRENT_VERSION,
            data,
        }
    }
}

#[derive(Serialize, Deserialize)]
enum FileVersion {
    V1,
}

fn save(state: &State) -> Result<(), Error> {
    let data = serde_json::to_string(state)?;
    let traw_file = TrawFile::new(data);
    let _ = std::fs::write("unnamed.traw", serde_json::to_string(&traw_file)?);
    Ok(())
}
