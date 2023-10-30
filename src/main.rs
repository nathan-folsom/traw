use std::io::stdout;

use crossterm::{
    event::{self, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};

fn main() -> std::io::Result<()> {
    // or using functions
    let _ = init()?;

    loop {
        match event::read()? {
            event::Event::Key(key) => {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
            _ => {}
        }
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
        SetForegroundColor(Color::Blue),
        Print("Styled text here."),
        ResetColor
    )?;

    Ok(())
}

fn end() -> std::io::Result<()> {
    execute!(stdout(), terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
