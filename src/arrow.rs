use std::cmp::Ordering::{self, Equal, Greater, Less};

use serde::{Deserialize, Serialize};

use crate::{
    characters::{
        ARROW_DOWN, ARROW_LEFT, ARROW_RIGHT, ARROW_UP, CORNER_1, CORNER_2, CORNER_3, CORNER_4,
        HORIZONTAL_BAR, VERTICAL_BAR,
    },
    draw::{Color, CursorIntersect, Draw, EdgeIntersection, Intersection, Point},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Arrow {
    pub points: Vec<(i32, i32)>,
}

impl Arrow {
    pub fn init() -> Self {
        Self { points: vec![] }
    }

    pub fn update(&mut self, (cursor_x, cursor_y): (u16, u16)) {
        if self.points.len() > 1
            && self.points[self.points.len() - 2] == (cursor_x as i32, cursor_y as i32)
        {
            self.points.pop();
        } else {
            self.points.push((cursor_x as i32, cursor_y as i32));
        }
    }
}

impl Draw for Arrow {
    fn draw(&self) -> std::io::Result<Vec<Point<i32>>> {
        let hover = self.hovered()?;
        let mut points = vec![];

        let foreground = Color::Border;
        let background = {
            if hover {
                Color::BorderBackgroundHover
            } else {
                Color::BorderBackground
            }
        };

        let arrow_spacing = {
            if self.points.len() < 5 {
                2
            } else {
                5
            }
        };

        self.points.iter().enumerate().for_each(|(i, point)| {
            if i == 0 {
                return;
            }
            let prev = self.points[i - 1];
            if i == self.points.len() - 1 {
                let is_vertical = point.1 != prev.1 && point.0 == prev.0;
                points.push(Point {
                    x: point.0,
                    y: point.1,
                    character: match is_vertical {
                        true => VERTICAL_BAR,
                        false => HORIZONTAL_BAR,
                    },
                    foreground,
                    background,
                });
                return;
            }

            points.push(Point {
                x: point.0,
                y: point.1,
                character: get_char(&prev, point, &self.points[i + 1], i % arrow_spacing == 0),
                foreground,
                background,
            });
        });

        Ok(points)
    }
}

impl CursorIntersect for Arrow {
    fn get_intersection(&self, x: &i32, y: &i32) -> crate::draw::Intersection {
        for (x_1, y_1) in &self.points {
            if x != x_1 || y != y_1 {
                continue;
            }
            return Intersection::Edge(EdgeIntersection::Side);
        }
        Intersection::None
    }
}

fn get_char(prev: &(i32, i32), current: &(i32, i32), next: &(i32, i32), use_arrow: bool) -> char {
    let get_arrow = |normal: char, arrow: char| match use_arrow {
        false => normal,
        true => arrow,
    };
    match (get_direction(prev, current), get_direction(current, next)) {
        ((Equal, Greater), (Equal, Greater)) => get_arrow(VERTICAL_BAR, ARROW_UP),
        ((Equal, Less), (Equal, Less)) => get_arrow(VERTICAL_BAR, ARROW_DOWN),
        ((Greater, Equal), (Greater, Equal)) => get_arrow(HORIZONTAL_BAR, ARROW_LEFT),
        ((Less, Equal), (Less, Equal)) => get_arrow(HORIZONTAL_BAR, ARROW_RIGHT),
        ((Greater, Equal), (Equal, Less)) | ((Equal, Greater), (Less, Equal)) => CORNER_3,
        ((Less, Equal), (Equal, Greater)) | ((Equal, Less), (Greater, Equal)) => CORNER_1,
        ((Less, Equal), (Equal, Less)) | ((Equal, Greater), (Greater, Equal)) => CORNER_4,
        ((Equal, Less), (Less, Equal)) | ((Greater, Equal), (Equal, Greater)) => CORNER_2,
        _ => ' ',
    }
}

fn get_direction(a: &(i32, i32), b: &(i32, i32)) -> (Ordering, Ordering) {
    (a.0.cmp(&b.0), a.1.cmp(&b.1))
}

#[cfg(test)]
mod test {
    use crate::characters::*;
    use crate::draw::Draw;

    use super::{get_char, Arrow};

    #[test]
    fn should_get_horizontal_bar() {
        assert_eq!(HORIZONTAL_BAR, get_char(&(0, 0), &(1, 0), &(2, 0), false));
    }

    #[test]
    fn should_get_vertical_bar() {
        assert_eq!(VERTICAL_BAR, get_char(&(0, 0), &(0, 1), &(0, 2), false));
    }

    #[test]
    fn should_get_corner_1() {
        assert_eq!(CORNER_1, get_char(&(0, 1), &(1, 1), &(1, 0), false));
        assert_eq!(CORNER_1, get_char(&(1, 0), &(1, 1), &(0, 1), false));
    }

    #[test]
    fn should_get_corner_2() {
        assert_eq!(CORNER_2, get_char(&(0, 0), &(0, 1), &(1, 1), false));
        assert_eq!(CORNER_2, get_char(&(1, 1), &(0, 1), &(0, 0), false));
    }

    #[test]
    fn should_get_corner_3() {
        assert_eq!(CORNER_3, get_char(&(0, 1), &(0, 0), &(1, 0), false));
        assert_eq!(CORNER_3, get_char(&(1, 0), &(0, 0), &(0, 1), false));
    }

    #[test]
    fn should_get_corner_4() {
        assert_eq!(CORNER_4, get_char(&(0, 0), &(1, 0), &(1, 1), false));
        assert_eq!(CORNER_4, get_char(&(1, 1), &(1, 0), &(0, 0), false));
    }

    #[test]
    fn should_get_down_arrow() {
        assert_eq!(ARROW_DOWN, get_char(&(0, 0), &(0, 1), &(0, 2), true));
    }

    #[test]
    fn should_get_up_arrow() {
        assert_eq!(ARROW_UP, get_char(&(0, 2), &(0, 1), &(0, 0), true));
    }

    #[test]
    fn should_get_left_arrow() {
        assert_eq!(ARROW_LEFT, get_char(&(2, 0), &(1, 0), &(0, 0), true));
    }

    #[test]
    fn should_get_right_arrow() {
        assert_eq!(ARROW_RIGHT, get_char(&(0, 0), &(1, 0), &(2, 0), true));
    }

    #[test]
    fn should_not_render_first_position() {
        let mut arrow = Arrow::init();
        arrow.points = vec![(0, 0), (1, 0), (2, 0)];

        let render = arrow.draw().unwrap();
        assert_eq!(render.len(), arrow.points.len() - 1);
    }

    #[test]
    fn should_remove_point_when_revisiting_previous_point() {
        let mut arrow = Arrow::init();
        arrow.points = vec![(0, 0), (1, 0), (2, 0)];
        arrow.update((1, 0));
        let expected = vec![(0, 0), (1, 0)];
        assert_eq!(arrow.points, expected);
    }
}
