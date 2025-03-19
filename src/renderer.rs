use std::io::{stdout, Write};

use cli_clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::{
    queue,
    style::{Print, SetBackgroundColor, SetForegroundColor},
};

use crate::{
    components::intersections::Intersections,
    cursor::{cursor_pos, restore_position, save_position, set_position},
    debug_panel::{DebugPanel, DEBUG_PANEL_HEIGHT},
    draw::{Color, Draw, DrawOverlay, DrawSticky, OverlayPoint, Point},
    grid_background::GridBackground,
    mode::{Anchor, Mode, Selection},
    rectangle::Drag,
    state::State,
    status_bar::StatusBar,
};

pub struct Renderer {
    state: Vec<Vec<StatePoint>>,
    prev_state: Vec<Vec<StatePoint>>,
    status_bar: StatusBar,
    grid_background: GridBackground,
    debug_panel: DebugPanel,
    width: u16,
    height: u16,
    is_first_frame: bool,
}

#[derive(PartialEq, Clone)]
struct StatePoint {
    character: char,
    foreground: Color,
    background: Color,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            state: vec![],
            prev_state: vec![],
            status_bar: Default::default(),
            grid_background: GridBackground::new(),
            debug_panel: Default::default(),
            width,
            height,
            is_first_frame: true,
        }
    }

    pub fn handle_yank(&self, selection: &Selection) {
        let mut ctx = ClipboardContext::new().unwrap();
        let mut content = vec![];
        for row in 0..selection.height {
            for col in 0..selection.width {
                let x = col + selection.x;
                let y = row + selection.y;
                let StatePoint {
                    character,
                    foreground,
                    background,
                } = self.state[x as usize][y as usize];
                let is_background = matches!(foreground, Color::Grid)
                    && matches!(background, Color::EmptyBackground);
                if is_background {
                    // Don't output background characters, they are purely aesthetic and won't
                    // make as much visual sense without the whole window for context
                    continue;
                }
                content.push(character);
            }
            content.push('\n');
        }
        ctx.set_contents(content.iter().collect()).unwrap();
    }

    pub fn render_frame(&mut self, state: &mut State) -> std::io::Result<()> {
        self.status_bar.update(&state.mode, {
            if state.debug_enabled {
                DEBUG_PANEL_HEIGHT as u16
            } else {
                0
            }
        })?;
        self.start_frame();
        self.render(self.grid_background.draw()?)?;
        self.render_sticky(self.status_bar.draw()?)?;
        if state.debug_enabled {
            self.render_sticky(self.debug_panel.draw()?)?;
        }
        self.render(state.draw()?)?;
        match &mut state.mode {
            Mode::Normal => {}
            Mode::DrawRectangle(rect, anchor) => {
                rect.drag_corner(anchor)?;
                self.render(rect.draw()?)?;
            }
            Mode::Text(rect) => {
                self.render(rect.draw()?)?;
            }
            Mode::DrawArrow(arrow) => {
                arrow.update(cursor_pos());
                self.render(arrow.draw()?)?;
            }
            Mode::Select(selection) => {
                selection.drag_corner(&mut Anchor::BottomRight)?;
                self.render_overlay(selection)?;
            }
        }
        self.render(Intersections::new(state).draw()?)?;
        self.render_overlay(state)?;
        self.finish_frame()?;
        if self.is_first_frame {
            self.is_first_frame = false;
        }
        stdout().flush()?;

        Ok(())
    }

    pub fn start_frame(&mut self) {
        let mut empty = vec![];
        for _ in 0..self.width {
            let mut cols = vec![];
            for _ in 0..self.height {
                cols.push(StatePoint {
                    character: ' ',
                    foreground: Color::Empty,
                    background: Color::EmptyBackground,
                });
            }
            empty.push(cols.clone());
        }
        if self.prev_state.is_empty() {
            self.prev_state = empty.clone();
        } else {
            std::mem::swap(&mut self.prev_state, &mut self.state);
        }
        self.state = empty;
    }

    pub fn render(&mut self, points: Vec<Point<i32>>) -> std::io::Result<()> {
        for point in points {
            self.draw_at(point)?;
        }
        Ok(())
    }

    pub fn render_sticky(&mut self, points: Vec<Point<u16>>) -> std::io::Result<()> {
        for point in points {
            self.draw_at(point.into())?;
        }
        Ok(())
    }

    fn draw_at(&mut self, point: Point<i32>) -> std::io::Result<()> {
        let Point {
            x,
            y,
            character,
            foreground,
            background,
        } = point;
        self.state[x as usize][y as usize] = StatePoint {
            character,
            foreground,
            background,
        };

        Ok(())
    }

    pub fn render_overlay(&mut self, overlay: &impl DrawOverlay) -> std::io::Result<()> {
        let (points, foreground, background) = overlay.draw_overlay()?;
        for OverlayPoint { x, y } in points {
            let point = &mut self.state[x as usize][y as usize];
            if let Some(fg) = foreground {
                point.foreground = fg;
            }
            if let Some(bg) = background {
                point.background = bg;
            }
        }
        Ok(())
    }

    pub fn finish_frame(&self) -> std::io::Result<()> {
        save_position();
        self.state
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, point)| {
                        let prev = &self.prev_state[x][y];
                        if point != prev || self.is_first_frame {
                            set_position(x as u16, y as u16);
                            queue!(
                                stdout(),
                                SetForegroundColor(point.foreground.into()),
                                SetBackgroundColor(point.background.into()),
                                Print(point.character)
                            )?;
                        }
                        std::io::Result::Ok(())
                    })
                    .collect::<std::io::Result<Vec<_>>>()
            })
            .collect::<std::io::Result<Vec<_>>>()?;
        restore_position();
        Ok(())
    }
}
