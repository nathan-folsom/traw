use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Clone, Debug, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Vec2 { x, y }
    }
}

impl From<Vec2<u16>> for Vec2<i32> {
    fn from(value: Vec2<u16>) -> Self {
        Vec2 {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl From<Vec2<i32>> for Vec2<u16> {
    fn from(value: Vec2<i32>) -> Self {
        Vec2 {
            x: value.x as u16,
            y: value.y as u16,
        }
    }
}

impl From<(i32, i32)> for Vec2<u16> {
    fn from((x, y): (i32, i32)) -> Self {
        Vec2 {
            x: x as u16,
            y: y as u16,
        }
    }
}

impl From<(usize, usize)> for Vec2<u16> {
    fn from((x, y): (usize, usize)) -> Self {
        Vec2 {
            x: x as u16,
            y: y as u16,
        }
    }
}

impl<T: Eq + Ord> Ord for Vec2<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.y.cmp(&other.y), self.x.cmp(&other.x)) {
            (Ordering::Less, _) => Ordering::Less,
            (Ordering::Equal, ord) => ord,
            (Ordering::Greater, _) => Ordering::Greater,
        }
    }
}

impl<T: Eq + Ord> PartialOrd for Vec2<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PartialEq> PartialEq for Vec2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
