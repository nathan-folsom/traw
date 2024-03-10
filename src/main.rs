use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute, queue,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use debug_panel::DebugPanel;
use draw::Renderer;
use mode::Mode;
use persistence::{load, save};
use rectangle::Drag;
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
    init()?;
    let mut state = State::init();
    let (width, height) = terminal::size()?;
    let mut renderer = Renderer::new(width, height);
    let mut status_bar = StatusBar::default();
    let debug_panel = DebugPanel::default();
    let path_arg = std::env::args().nth(1);
    let mut debug_enabled = false;

    let mut file_name = "unnamed.traw".to_string();

    if let Some(path) = path_arg {
        file_name = path;
        state = load(&file_name)?;
        status_bar.update(&state.mode, 0)?;
        renderer.render_sticky(&status_bar)?;
        for shape in &state.shapes {
            renderer.render(shape)?;
        }
        stdout().flush()?;
    }

    loop {
        if let event::Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char(key) => match &mut state.mode {
                    Mode::Text(rect) => {
                        rect.on_char(key)?;
                    }
                    Mode::Normal => match key {
                        'q' => break,
                        's' => save(&state, &file_name)?,
                        'i' => state.handle_insert()?,
                        'r' => state.handle_drag()?,
                        'x' => state.handle_delete()?,
                        'v' => state.handle_select()?,
                        'd' => debug_enabled = !debug_enabled,
                        _ => handle_motions(key)?,
                    },
                    Mode::Select(selection) => {
                        if key == 'y' {
                            renderer.handle_yank(selection);
                        }
                        handle_motions(key)?;
                    }
                    _ => {
                        handle_motions(key)?;
                    }
                },

                KeyCode::Enter => {
                    if let Mode::Select(selection) = &state.mode {
                        renderer.handle_yank(selection);
                    }
                    state.handle_enter()?;
                }
                KeyCode::Backspace => state.handle_backspace()?,
                _ => {}
            }
        }

        status_bar.update(&state.mode, {
            if debug_enabled {
                10
            } else {
                0
            }
        })?;
        renderer.start_frame();
        renderer.render_sticky(&status_bar)?;
        if debug_enabled {
            renderer.render_sticky(&debug_panel)?;
        }
        renderer.render(&state)?;

        match &mut state.mode {
            Mode::Normal => {}
            Mode::DrawRectangle(rect, anchor) => {
                rect.drag_corner(anchor)?;
                renderer.render(rect)?;
            }
            Mode::Text(rect) => {
                renderer.render(rect)?;
            }
            Mode::DrawArrow(arrow) => {
                arrow.update(cursor::position()?);
                renderer.render(arrow)?;
            }
            Mode::Select(selection) => {
                selection.drag_corner(&mode::Anchor::BottomRight)?;
                renderer.render_overlay(selection)?;
            }
        }

        renderer.finish_frame()?;

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

fn handle_motions(key: char) -> std::io::Result<()> {
    match key {
        'h' => {
            queue!(stdout(), cursor::MoveLeft(1))?;
        }
        'j' => {
            queue!(stdout(), cursor::MoveDown(1))?;
        }
        'k' => {
            queue!(stdout(), cursor::MoveUp(1))?;
        }
        'l' => {
            queue!(stdout(), cursor::MoveRight(1))?;
        }
        _ => {}
    }

    Ok(())
}
