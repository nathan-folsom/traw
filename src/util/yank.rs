use cli_clipboard::{ClipboardContext, ClipboardProvider};

use crate::{
    mode::Selection,
    renderer::{self, Renderer},
};

pub fn handle_yank(renderer: &Renderer, selection: &Selection) {
    let mut ctx = ClipboardContext::new().unwrap();
    let mut content = vec![];
    for row in 0..selection.height {
        for col in 0..selection.width {
            let x = col + selection.x;
            let y = row + selection.y;
            let cell = &renderer.state[x as usize][y as usize];
            if cell.shape_id.is_none() {
                // Only output drawn shapes, no background or other characters
                continue;
            }
            content.push(cell.character);
        }
        content.push('\n');
    }
    ctx.set_contents(content.iter().collect()).unwrap();
}
