use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    io::Result,
};

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

    const FG: Color = Color::Border;
    const BG: Color = Color::BorderBackground;
    const BG_HOVER: Color = Color::BorderBackgroundHover;

    fn get_endpoint(&self, point: &(i32, i32), neighbor: &(i32, i32)) -> Result<Point<i32>> {
        self.get_point(
            point,
            match point.1 != neighbor.1 && point.0 == neighbor.0 {
                true => VERTICAL_BAR,
                false => HORIZONTAL_BAR,
            },
        )
    }

    fn get_point(&self, (x, y): &(i32, i32), c: char) -> Result<Point<i32>> {
        let (foreground, background) = self.get_color()?;
        Ok(Point {
            x: *x,
            y: *y,
            character: c,
            foreground,
            background,
        })
    }

    fn get_color(&self) -> Result<(Color, Color)> {
        if self.hovered()? {
            Ok((Self::FG, Self::BG_HOVER))
        } else {
            Ok((Self::FG, Self::BG))
        }
    }

    fn get_char(
        prev: &(i32, i32),
        current: &(i32, i32),
        next: &(i32, i32),
        try_arrow: &mut bool,
    ) -> char {
        let mut get_arrow = |normal: char, arrow: char| match try_arrow {
            false => normal,
            true => {
                *try_arrow = false;
                arrow
            }
        };
        match (
            Self::get_direction(prev, current),
            Self::get_direction(current, next),
        ) {
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
}

impl Draw for Arrow {
    fn draw(&self) -> std::io::Result<Vec<Point<i32>>> {
        let mut points = vec![];
        let mut add_arrow = false;
        self.points
            .iter()
            .enumerate()
            .map(|(i, point)| {
                let prev = match i {
                    0 => None,
                    n => self.points.get(n - 1),
                };
                let next = self.points.get(i + 1);
                if i == self.points.len() / 2 {
                    add_arrow = true;
                }
                match (prev, next) {
                    (None, Some(p)) | (Some(p), None) => points.push(self.get_endpoint(point, p)?),
                    (Some(p), Some(n)) => {
                        points.push(
                            self.get_point(point, Self::get_char(p, point, n, &mut add_arrow))?,
                        );
                    }
                    _ => {}
                };
                Result::Ok(())
            })
            .collect::<Result<Vec<_>>>()?;

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

#[cfg(test)]
mod test {
    use crate::characters::*;

    use super::Arrow;

    #[test]
    fn should_get_horizontal_bar() {
        assert_eq!(
            HORIZONTAL_BAR,
            Arrow::get_char(&(0, 0), &(1, 0), &(2, 0), &mut false)
        );
    }

    #[test]
    fn should_get_vertical_bar() {
        assert_eq!(
            VERTICAL_BAR,
            Arrow::get_char(&(0, 0), &(0, 1), &(0, 2), &mut false)
        );
    }

    #[test]
    fn should_get_corner_1() {
        assert_eq!(
            CORNER_1,
            Arrow::get_char(&(0, 1), &(1, 1), &(1, 0), &mut false)
        );
        assert_eq!(
            CORNER_1,
            Arrow::get_char(&(1, 0), &(1, 1), &(0, 1), &mut false)
        );
    }

    #[test]
    fn should_get_corner_2() {
        assert_eq!(
            CORNER_2,
            Arrow::get_char(&(0, 0), &(0, 1), &(1, 1), &mut false)
        );
        assert_eq!(
            CORNER_2,
            Arrow::get_char(&(1, 1), &(0, 1), &(0, 0), &mut false)
        );
    }

    #[test]
    fn should_get_corner_3() {
        assert_eq!(
            CORNER_3,
            Arrow::get_char(&(0, 1), &(0, 0), &(1, 0), &mut false)
        );
        assert_eq!(
            CORNER_3,
            Arrow::get_char(&(1, 0), &(0, 0), &(0, 1), &mut false)
        );
    }

    #[test]
    fn should_get_corner_4() {
        assert_eq!(
            CORNER_4,
            Arrow::get_char(&(0, 0), &(1, 0), &(1, 1), &mut false)
        );
        assert_eq!(
            CORNER_4,
            Arrow::get_char(&(1, 1), &(1, 0), &(0, 0), &mut false)
        );
    }

    #[test]
    fn should_get_down_arrow() {
        assert_eq!(
            ARROW_DOWN,
            Arrow::get_char(&(0, 0), &(0, 1), &(0, 2), &mut true)
        );
    }

    #[test]
    fn should_get_up_arrow() {
        assert_eq!(
            ARROW_UP,
            Arrow::get_char(&(0, 2), &(0, 1), &(0, 0), &mut true)
        );
    }

    #[test]
    fn should_get_left_arrow() {
        assert_eq!(
            ARROW_LEFT,
            Arrow::get_char(&(2, 0), &(1, 0), &(0, 0), &mut true)
        );
    }

    #[test]
    fn should_get_right_arrow() {
        assert_eq!(
            ARROW_RIGHT,
            Arrow::get_char(&(0, 0), &(1, 0), &(2, 0), &mut true)
        );
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
