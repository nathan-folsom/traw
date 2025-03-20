use std::io::stdout;

use components::{
    debug_panel::{DebugPanel, DEBUG_PANEL_HEIGHT},
    grid_background::GridBackground,
    intersections::Intersections,
    rectangle::Drag,
    status_bar::StatusBar,
};
use crossterm::{
    event::{self, KeyCode, KeyModifiers},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use cursor::cursor_pos;
use draw::{Draw, DrawSticky};
use mode::{Anchor, Mode};
use motion_state::MotionState;
use persistence::{load, save};
use renderer::Renderer;
use state::State;

mod characters;
mod components;
mod cursor;
mod cursor_guide;
mod draw;
mod mode;
mod motion_state;
mod mutate;
mod persistence;
mod renderer;
mod shape;
mod state;

fn main() -> std::io::Result<()> {
    init()?;
    let mut state = State::init();
    let mut motion_state = MotionState::new();
    let (width, height) = terminal::size()?;
    let mut renderer = Renderer::new(width, height);
    let path_arg = std::env::args().nth(1);

    let mut file_name = "unnamed.traw".to_string();

    if let Some(path) = path_arg {
        file_name = path;
        state = load(&file_name)?;
    }

    render(&mut renderer, &mut state)?;

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
                        'r' => {
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                                state.redo();
                            } else {
                                state.handle_drag()?;
                            }
                        }
                        'x' => state.handle_delete()?,
                        'v' => state.handle_select()?,
                        'd' => state.debug_enabled = !state.debug_enabled,
                        'u' => state.undo(),
                        _ => motion_state.handle_motions(key)?,
                    },
                    Mode::Select(selection) => {
                        if key == 'y' {
                            renderer.handle_yank(selection);
                        }
                        motion_state.handle_motions(key)?;
                    }
                    _ => {
                        motion_state.handle_motions(key)?;
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

        render(&mut renderer, &mut state)?;
    }

    cleanup()?;

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

fn cleanup() -> std::io::Result<()> {
    execute!(stdout(), terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn render(renderer: &mut Renderer, state: &mut State) -> std::io::Result<()> {
    renderer.render_frame(|r| {
        r.render(GridBackground::new().draw()?)?;
        r.render_sticky(
            StatusBar::new(&state.mode, {
                if state.debug_enabled {
                    DEBUG_PANEL_HEIGHT as u16
                } else {
                    0
                }
            })
            .draw()?,
        )?;
        if state.debug_enabled {
            r.render_sticky(DebugPanel {}.draw()?)?;
        }
        r.render(state.draw()?)?;
        match &mut state.mode {
            Mode::Normal => {}
            Mode::DrawRectangle(rect, anchor) => {
                rect.drag_corner(anchor)?;
                r.render(rect.draw()?)?;
            }
            Mode::Text(rect) => {
                r.render(rect.draw()?)?;
            }
            Mode::DrawArrow(arrow) => {
                arrow.update(cursor_pos());
                r.render(arrow.draw()?)?;
            }
            Mode::Select(selection) => {
                selection.drag_corner(&mut Anchor::BottomRight)?;
                r.render_overlay(selection)?;
            }
        }
        r.render(Intersections::new(state).draw()?)?;
        r.render_overlay(state)?;
        Ok(())
    })
}
