use serde::{Deserialize, Serialize};

use crate::{arrow::Arrow, rectangle::Rectangle};

#[derive(Serialize, Deserialize, Debug)]
pub enum Mode {
    Normal,
    DrawRectangle(Rectangle),
    DrawArrow(Arrow),
    Text(Rectangle),
}
