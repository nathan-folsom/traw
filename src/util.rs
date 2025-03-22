use std::cmp::Ordering;

#[derive(Clone, Debug, Eq)]
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
