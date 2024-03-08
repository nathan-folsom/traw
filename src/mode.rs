use serde::{Deserialize, Serialize};

use crate::{arrow::Arrow, rectangle::Rectangle};

#[derive(Serialize, Deserialize, Debug)]
pub enum Mode {
    Normal,
    DrawRectangle(Rectangle, Anchor),
    DrawArrow(Arrow),
    Text(Rectangle),
}

/// When resizing a rectangle, which corner is being dragged
#[derive(Serialize, Deserialize, Debug)]
pub enum Anchor {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}
