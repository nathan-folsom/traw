use std::{
    cmp::Ordering::{Equal, Greater, Less},
    io::Result,
};

use serde::{Deserialize, Serialize};

use crate::{
    characters::{
        ARROW_DOWN, ARROW_LEFT, ARROW_RIGHT, ARROW_UP, CORNER_1, CORNER_2, CORNER_3, CORNER_4,
        HORIZONTAL_BAR, VERTICAL_BAR,
    },
    cursor_guide::GuidePoint,
    draw::{Color, CursorIntersect, Draw, EdgeIntersection, Intersection, Point},
    shape_id::generate_shape_id,
    util::Vec2,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Arrow {
    pub points: Vec<Vec2<i32>>,
    pub shape_id: u32,
}

impl Arrow {
    pub fn init() -> Self {
        Self {
            points: vec![],
            shape_id: generate_shape_id(),
        }
    }

    pub fn update(&mut self, position: Vec2<i32>) {
        if self.points.len() > 1 && self.points[self.points.len() - 2] == position {
            self.points.pop();
        } else {
            self.points.push(position);
        }
    }

    const FG: Color = Color::Border;
    const BG: Color = Color::BorderBackground;

    fn get_endpoint(&self, point: &Vec2<i32>, neighbor: &Vec2<i32>) -> Result<Point<i32>> {
        self.get_point(
            point,
            match point.y != neighbor.y && point.x == neighbor.x {
                true => VERTICAL_BAR,
                false => HORIZONTAL_BAR,
            },
        )
    }

    fn get_point(&self, Vec2 { x, y }: &Vec2<i32>, c: char) -> Result<Point<i32>> {
        Ok(Point {
            x: *x,
            y: *y,
            character: c,
            foreground: Self::FG,
            background: Self::BG,
        })
    }

    fn get_char(
        prev: &Vec2<i32>,
        current: &Vec2<i32>,
        next: &Vec2<i32>,
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
            (prev.x.cmp(&current.x), prev.y.cmp(&current.y)),
            (current.x.cmp(&next.x), current.y.cmp(&next.y)),
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
    fn get_intersection(&self, p: &Vec2<i32>) -> crate::draw::Intersection {
        for point in &self.points {
            if point != p {
                continue;
            }
            return Intersection::Edge(EdgeIntersection::Side);
        }
        Intersection::None
    }
}

impl GuidePoint for Arrow {
    fn get_intersection_points(&self) -> Vec<Vec2<i32>> {
        vec![self.points.first(), self.points.last()]
            .into_iter()
            .flatten()
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::{characters::*, util::Vec2};

    use super::Arrow;

    #[test]
    fn should_get_horizontal_bar() {
        assert_eq!(
            HORIZONTAL_BAR,
            Arrow::get_char(&(0, 0).into(), &(1, 0).into(), &(2, 0).into(), &mut false)
        );
    }

    #[test]
    fn should_get_vertical_bar() {
        assert_eq!(
            VERTICAL_BAR,
            Arrow::get_char(&(0, 0).into(), &(0, 1).into(), &(0, 2).into(), &mut false)
        );
    }

    #[test]
    fn should_get_corner_1() {
        assert_eq!(
            CORNER_1,
            Arrow::get_char(&(0, 1).into(), &(1, 1).into(), &(1, 0).into(), &mut false)
        );
        assert_eq!(
            CORNER_1,
            Arrow::get_char(&(1, 0).into(), &(1, 1).into(), &(0, 1).into(), &mut false)
        );
    }

    #[test]
    fn should_get_corner_2() {
        assert_eq!(
            CORNER_2,
            Arrow::get_char(&(0, 0).into(), &(0, 1).into(), &(1, 1).into(), &mut false)
        );
        assert_eq!(
            CORNER_2,
            Arrow::get_char(&(1, 1).into(), &(0, 1).into(), &(0, 0).into(), &mut false)
        );
    }

    #[test]
    fn should_get_corner_3() {
        assert_eq!(
            CORNER_3,
            Arrow::get_char(&(0, 1).into(), &(0, 0).into(), &(1, 0).into(), &mut false)
        );
        assert_eq!(
            CORNER_3,
            Arrow::get_char(&(1, 0).into(), &(0, 0).into(), &(0, 1).into(), &mut false)
        );
    }

    #[test]
    fn should_get_corner_4() {
        assert_eq!(
            CORNER_4,
            Arrow::get_char(&(0, 0).into(), &(1, 0).into(), &(1, 1).into(), &mut false)
        );
        assert_eq!(
            CORNER_4,
            Arrow::get_char(&(1, 1).into(), &(1, 0).into(), &(0, 0).into(), &mut false)
        );
    }

    #[test]
    fn should_get_down_arrow() {
        assert_eq!(
            ARROW_DOWN,
            Arrow::get_char(&(0, 0).into(), &(0, 1).into(), &(0, 2).into(), &mut true)
        );
    }

    #[test]
    fn should_get_up_arrow() {
        assert_eq!(
            ARROW_UP,
            Arrow::get_char(&(0, 2).into(), &(0, 1).into(), &(0, 0).into(), &mut true)
        );
    }

    #[test]
    fn should_get_left_arrow() {
        assert_eq!(
            ARROW_LEFT,
            Arrow::get_char(&(2, 0).into(), &(1, 0).into(), &(0, 0).into(), &mut true)
        );
    }

    #[test]
    fn should_get_right_arrow() {
        assert_eq!(
            ARROW_RIGHT,
            Arrow::get_char(&(0, 0).into(), &(1, 0).into(), &(2, 0).into(), &mut true)
        );
    }

    #[test]
    fn should_remove_point_when_revisiting_previous_point() {
        let mut arrow = Arrow::init();
        arrow.points = vec![Vec2::new(0, 0), Vec2::new(1, 0), Vec2::new(2, 0)];
        arrow.update((1, 0).into());
        let expected = vec![Vec2::new(0, 0), Vec2::new(1, 0)];
        assert_eq!(arrow.points, expected);
    }
}
