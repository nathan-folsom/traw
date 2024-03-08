use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent},
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
        if let event::Event::Key(key_event) = event::read()? {
            match state.mode {
                Mode::Text(_) => {}
                _ => handle_motions(key_event)?,
            }

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
                        _ => {}
                    },
                    Mode::Select(selection) => {
                        if key == 'y' {
                            renderer.handle_yank(selection);
                        }
                    }
                    _ => {}
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

        status_bar.update(&state.mode)?;
        renderer.start_frame();
        renderer.render_sticky(&status_bar)?;
        renderer.render_sticky(&debug_panel)?;
        for shape in &state.shapes {
            renderer.render(shape)?;
        }

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

fn handle_motions(event: KeyEvent) -> std::io::Result<()> {
    if let KeyCode::Char(key) = event.code {
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
    }

    Ok(())
}
