use std::io::{stdout, Write};

use arrow::Arrow;
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use draw::Renderer;
use mode::Mode;
use rectangle::{Rectangle, RectangleIntersection};
use status_bar::StatusBar;

mod arrow;
mod draw;
mod mode;
mod rectangle;
mod status_bar;

fn main() -> std::io::Result<()> {
    let _ = init()?;
    let mut rectangles = vec![];
    let (width, height) = terminal::size()?;
    let mut renderer = Renderer::new(width, height);
    let mut mode = Mode::Normal;
    let mut status_bar = StatusBar::default();

    loop {
        match event::read()? {
            event::Event::Key(key_event) => {
                match &mut mode {
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
                    KeyCode::Char(key) => match &mut mode {
                        Mode::Text(rect) => {
                            rect.on_char(key);
                            queue!(stdout(), cursor::MoveRight(1))?;
                        }
                        Mode::Normal | Mode::DrawRectangle(_) => match key {
                            'q' => break,
                            'i' => match mode {
                                Mode::DrawRectangle(rect) => {
                                    mode = Mode::Text(rect);
                                }
                                Mode::Normal => {
                                    let (x, y) = cursor::position()?;
                                    let intersection = get_rectangle_intersection(&rectangles)?;

                                    match intersection {
                                        RectangleIntersection::None => {
                                            mode = Mode::DrawRectangle(Rectangle::new_at(
                                                x as i32, y as i32,
                                            ));
                                        }
                                        RectangleIntersection::Edge => {
                                            mode = Mode::DrawArrow(Arrow {});
                                        }
                                        RectangleIntersection::Inner => {
                                            todo!();
                                            // mode = Mode::Text(());
                                        }
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    },

                    KeyCode::Enter => match mode {
                        Mode::DrawRectangle(rect) => {
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
                }
            }
            _ => {}
        }

        status_bar.update(&mode)?;
        renderer.render_sticky(&status_bar)?;

        match &mut mode {
            Mode::Normal => {}
            Mode::DrawRectangle(rect) => {
                rect.update()?;
                renderer.render(rect)?;
            }
            Mode::Text(rect) => {
                renderer.render(rect)?;
            }
            _ => {}
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

fn get_rectangle_intersection(
    rectangles: &Vec<Rectangle>,
) -> std::io::Result<RectangleIntersection> {
    for rectangle in rectangles {
        let intersection = rectangle.get_intersection();

        if intersection.is_ok() {
            return intersection;
        }
    }

    Ok(RectangleIntersection::None)
}

fn cursor_on_rectangle_border(rectangles: &Vec<Rectangle>) -> std::io::Result<bool> {
    for rectangle in rectangles {
        let (cursor_x, cursor_y) = cursor::position()?;
        let cursor_on_top_border = cursor_y == rectangle.y as u16;
        let cursor_on_bottom_border = cursor_y as i32 == rectangle.y + rectangle.height - 1;
    }

    Ok(false)
}

fn cursor_in_rectangle(rectangles: &Vec<Rectangle>) -> std::io::Result<bool> {
    for rectangle in rectangles {
        let (cursor_x, cursor_y) = cursor::position()?;
    }

    Ok(false)
}
