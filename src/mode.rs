use crate::{arrow::Arrow, rectangle::Rectangle};

pub enum Mode {
    Normal,
    DrawRectangle(Rectangle),
    DrawArrow(Arrow),
    Text(Rectangle),
}
