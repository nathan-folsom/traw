use std::io::Result;

use serde::{Deserialize, Serialize};

use crate::{
    components::{
        arrow::Arrow,
        rectangle::{Drag, Rectangle},
    },
    draw::{Color, DrawOverlay, OverlayPoint},
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum Mode {
    #[default]
    Normal,
    DrawRectangle(Rectangle, Anchor),
    DrawArrow(Arrow),
    Select(Selection),
    Text(Rectangle),
}

impl Mode {
    pub fn is_normal(&self) -> bool {
        matches!(self, Mode::Normal)
    }
}

/// When resizing a rectangle, which corner is being dragged
#[derive(Serialize, Deserialize, Debug)]
pub enum Anchor {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Selection {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Drag for Selection {
    fn rect(&mut self) -> (&mut i32, &mut i32, &mut i32, &mut i32) {
        (&mut self.x, &mut self.y, &mut self.width, &mut self.height)
    }
}

impl DrawOverlay for Selection {
    fn draw_overlay(&self) -> Result<(Vec<OverlayPoint>, Option<Color>, Option<Color>)> {
        let mut points = vec![];
        let background = Some(Color::BorderBackgroundHover);

        for y in 0..self.height {
            for x in 0..self.width {
                points.push(OverlayPoint {
                    x: self.x + x,
                    y: self.y + y,
                });
            }
        }

        Ok((points, None, background))
    }
}
