use std::io::{stdout, Write};

use cli_clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::{
    queue,
    style::{Print, SetBackgroundColor, SetForegroundColor},
};

use crate::{
    characters::{INTERSECTION_DOWN, INTERSECTION_LEFT, INTERSECTION_RIGHT, INTERSECTION_UP},
    cursor::{cursor_pos, restore_position, save_position, set_position},
    debug_panel::{DebugPanel, DEBUG_PANEL_HEIGHT},
    draw::{
        Color, CursorIntersect, Draw, DrawOverlay, DrawSticky, Intersection, OverlayPoint, Point,
    },
    grid_background::GridBackground,
    mode::{Anchor, Mode, Selection},
    rectangle::Drag,
    shape::Shape,
    state::State,
    status_bar::StatusBar,
};

pub struct Renderer {
    state: Vec<Vec<(char, Color, Color)>>,
    prev_state: Vec<Vec<(char, Color, Color)>>,
    status_bar: StatusBar,
    grid_background: GridBackground,
    debug_panel: DebugPanel,
    width: u16,
    height: u16,
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
        }
    }

    pub fn start_frame(&mut self) {
        let mut empty = vec![];
        for _ in 0..self.width {
            let mut cols = vec![];
            for _ in 0..self.height {
                cols.push((' ', Color::Empty, Color::EmptyBackground));
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

    pub fn finish_frame(&self) -> std::io::Result<()> {
        save_position();
        self.state
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, (character, foreground, background))| {
                        let (prev_char, prev_foreground, prev_background) = self.prev_state[x][y];
                        if character != &prev_char
                            || foreground != &prev_foreground
                            || background != &prev_background
                        {
                            set_position(x as u16, y as u16);
                            queue!(
                                stdout(),
                                SetForegroundColor(foreground.into()),
                                SetBackgroundColor(background.into()),
                                Print(character)
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

    pub fn render_overlay(&mut self, overlay: &impl DrawOverlay) -> std::io::Result<()> {
        let (points, foreground, background) = overlay.draw_overlay()?;
        for OverlayPoint { x, y } in points {
            let (_, current_foreground, current_background) =
                &mut self.state[x as usize][y as usize];
            if let Some(fg) = foreground {
                *current_foreground = fg;
            }
            if let Some(bg) = background {
                *current_background = bg;
            }
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
        self.state[x as usize][y as usize] = (character, foreground, background);

        Ok(())
    }

    pub fn handle_yank(&self, selection: &Selection) {
        let mut ctx = ClipboardContext::new().unwrap();
        let mut content = vec![];
        for row in 0..selection.height {
            for col in 0..selection.width {
                let x = col + selection.x;
                let y = row + selection.y;
                let (character, foreground, background) = self.state[x as usize][y as usize];
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
        self.render_intersections(state)?;
        self.render_overlay(state)?;

        self.finish_frame()?;

        stdout().flush()?;
        Ok(())
    }

    fn render_intersections(&mut self, state: &State) -> std::io::Result<()> {
        let mut all_arrows = vec![];
        let mut all_rectangles = vec![];
        state.shapes.iter().for_each(|s| match s {
            Shape::Rectangle(r) => all_rectangles.push(r),
            Shape::Arrow(a) => all_arrows.push(a),
        });
        match &state.mode {
            Mode::DrawRectangle(rect, _) => {
                all_rectangles.push(rect);
            }
            Mode::Text(rect) => {
                all_rectangles.push(rect);
            }
            Mode::DrawArrow(arrow) => {
                all_arrows.push(arrow);
            }
            _ => {}
        }
        let mut intersection_points = vec![];
        let mut add_intersection_point =
            |point: Option<&(i32, i32)>, reference: Option<&(i32, i32)>| {
                if let Some((x, y)) = point {
                    all_rectangles.iter().for_each(|r| {
                        let intersection = r.get_intersection(x, y);
                        if let Intersection::Edge(_) = intersection {
                            if let Some((x_1, y_1)) = reference {
                                let character = {
                                    if x_1 > x {
                                        INTERSECTION_RIGHT
                                    } else if x_1 < x {
                                        INTERSECTION_LEFT
                                    } else if y_1 < y {
                                        INTERSECTION_UP
                                    } else if y_1 > y {
                                        INTERSECTION_DOWN
                                    } else {
                                        unreachable!("Reference point should always be different than endpoint")
                                    }
                                };
                                intersection_points.push(Point {
                                    x: *x,
                                    y: *y,
                                    character,
                                    foreground: Color::Border,
                                    background: Color::BorderBackground,
                                });
                            }
                        }
                    })
                }
            };
        all_arrows.iter().for_each(|a| {
            add_intersection_point(a.points.first(), a.points.get(1));
            if a.points.len() > 1 {
                add_intersection_point(a.points.last(), a.points.get(a.points.len() - 2));
            }
        });
        self.render(intersection_points)
    }
}
