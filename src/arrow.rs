use crossterm::cursor;
use serde::{Deserialize, Serialize};

use crate::{
    characters::{CORNER_1, CORNER_2, CORNER_3, CORNER_4, HORIZONTAL_BAR, VERTICAL_BAR},
    draw::{Color, Draw, Intersection, Point},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Arrow {
    pub points: Vec<(i32, i32)>,
    clear: Option<(i32, i32)>,
}

impl Arrow {
    pub fn init() -> Self {
        Self {
            points: vec![],
            clear: None,
        }
    }

    pub fn update(&mut self, (cursor_x, cursor_y): (u16, u16)) {
        if self.points.len() > 1
            && self.points[self.points.len() - 2] == (cursor_x as i32, cursor_y as i32)
        {
            self.clear = self.points.pop();
        } else {
            self.points.push((cursor_x as i32, cursor_y as i32));
        }
    }
}

impl Draw for Arrow {
    fn draw(&self, hover: bool) -> std::io::Result<Vec<Point<i32>>> {
        let mut points = vec![];

        let foreground = Color::Border;
        let background = {
            if hover {
                Color::BorderBackgroundHover
            } else {
                Color::BorderBackground
            }
        };

        if let Some((x, y)) = self.clear {
            points.push(Point {
                x,
                y,
                character: ' ',
                foreground: Color::Empty,
                background: Color::EmptyBackground,
            });
        }

        for i in 0..self.points.len() {
            let is_first_point = i == 0;

            if is_first_point {
                continue;
            }

            let point = self.points[i];
            let is_last_point = i == self.points.len() - 1;
            let prev = self.points[i - 1];

            if is_last_point {
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
                continue;
            }

            points.push(Point {
                x: point.0,
                y: point.1,
                character: get_char(&prev, &point, &self.points[i + 1]),
                foreground,
                background,
            });
        }

        Ok(points)
    }

    fn get_intersection(&self) -> std::io::Result<crate::draw::Intersection> {
        let (c_x, c_y) = cursor::position()?;

        for Point {
            x, y, character, ..
        } in self.draw(false)?
        {
            if x as u16 != c_x || y as u16 != c_y {
                continue;
            }

            if character == HORIZONTAL_BAR || character == VERTICAL_BAR {
                return Ok(Intersection::Edge(crate::draw::EdgeIntersection::Side));
            }

            return Ok(Intersection::Edge(crate::draw::EdgeIntersection::Corner(
                None,
            )));
        }

        Ok(Intersection::None)
    }

    fn clear(&self) -> std::io::Result<Vec<(i32, i32)>> {
        Ok(self.points.clone())
    }
}

fn get_char(prev: &(i32, i32), current: &(i32, i32), next: &(i32, i32)) -> char {
    let positive_y = prev.1 > current.1 || next.1 > current.1;
    let negative_y = prev.1 < current.1 || next.1 < current.1;
    let positive_x = prev.0 > current.0 || next.0 > current.0;
    let negative_x = prev.0 < current.0 || next.0 < current.0;

    if negative_y && negative_x {
        CORNER_1
    } else if negative_y && positive_x {
        CORNER_2
    } else if positive_x && positive_y {
        CORNER_3
    } else if negative_x && positive_y {
        CORNER_4
    } else if positive_y || negative_y {
        VERTICAL_BAR
    } else if positive_x || negative_x {
        HORIZONTAL_BAR
    } else {
        ' '
    }
}

#[cfg(test)]
mod test {
    use crate::characters::*;
    use crate::draw::Draw;

    use super::{get_char, Arrow};

    #[test]
    fn should_get_horizontal_bar() {
        let prev = (0, 0);
        let current = (1, 0);
        let next = (2, 0);

        let next_char = get_char(&prev, &current, &next);
        assert_eq!(HORIZONTAL_BAR, next_char);
    }

    #[test]
    fn should_get_vertical_bar() {
        let prev = (0, 0);
        let current = (0, 1);
        let next = (0, 2);

        let next_char = get_char(&prev, &current, &next);
        assert_eq!(VERTICAL_BAR, next_char);
    }

    #[test]
    fn should_get_corner_1() {
        let prev = (0, 1);
        let current = (1, 1);
        let next = (1, 0);

        let next_char = get_char(&prev, &current, &next);
        assert_eq!(CORNER_1, next_char);
    }

    #[test]
    fn should_get_corner_2() {
        let prev = (0, 0);
        let current = (0, 1);
        let next = (1, 1);

        let next_char = get_char(&prev, &current, &next);
        assert_eq!(CORNER_2, next_char);
    }

    #[test]
    fn should_get_corner_3() {
        let prev = (0, 1);
        let current = (0, 0);
        let next = (1, 0);

        let next_char = get_char(&prev, &current, &next);
        assert_eq!(CORNER_3, next_char);
    }

    #[test]
    fn should_get_corner_4() {
        let prev = (0, 0);
        let current = (1, 0);
        let next = (1, 1);

        let next_char = get_char(&prev, &current, &next);
        assert_eq!(CORNER_4, next_char);
    }

    #[test]
    fn should_not_render_first_position() {
        let mut arrow = Arrow::init();
        arrow.points = vec![(0, 0), (1, 0), (2, 0)];

        let render = arrow.draw(false).unwrap();
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
