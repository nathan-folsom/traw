use crate::{
    characters::{INTERSECTION_DOWN, INTERSECTION_LEFT, INTERSECTION_RIGHT, INTERSECTION_UP},
    draw::{Color, CursorIntersect, Draw, Intersection, Point},
    mode::Mode,
    shape::Shape,
    state::State,
};

pub struct Intersections {
    points: Vec<Point<i32>>,
}

impl Intersections {
    pub fn new(state: &State) -> Self {
        let mut all_arrows = vec![];
        let mut all_rectangles = vec![];
        state.shapes.iter().for_each(|s| match s {
            Shape::Rectangle(r) => all_rectangles.push(r),
            Shape::Arrow(a) => all_arrows.push(a),
        });
        match &state.mode {
            Mode::DrawRectangle(rect, _) => {
                all_rectangles.push(rect);
            }
            Mode::Text(rect) => {
                all_rectangles.push(rect);
            }
            Mode::DrawArrow(arrow) => {
                all_arrows.push(arrow);
            }
            _ => {}
        }
        let mut intersection_points = vec![];
        let mut add_intersection_point =
            |point: Option<&(i32, i32)>, reference: Option<&(i32, i32)>| {
                if let Some((x, y)) = point {
                    all_rectangles.iter().for_each(|r| {
                        let intersection = r.get_intersection(x, y);
                        if let Intersection::Edge(_) = intersection {
                            if let Some((x_1, y_1)) = reference {
                                let character = {
                                    if x_1 > x {
                                        INTERSECTION_RIGHT
                                    } else if x_1 < x {
                                        INTERSECTION_LEFT
                                    } else if y_1 < y {
                                        INTERSECTION_UP
                                    } else if y_1 > y {
                                        INTERSECTION_DOWN
                                    } else {
                                        unreachable!("Reference point should always be different than endpoint")
                                    }
                                };
                                intersection_points.push(Point {
                                    x: *x,
                                    y: *y,
                                    character,
                                    foreground: Color::Border,
                                    background: Color::BorderBackground,
                                });
                            }
                        }
                    })
                }
            };
        all_arrows.iter().for_each(|a| {
            add_intersection_point(a.points.first(), a.points.get(1));
            if a.points.len() > 1 {
                add_intersection_point(a.points.last(), a.points.get(a.points.len() - 2));
            }
        });
        Self {
            points: intersection_points,
        }
    }
}

impl Draw for Intersections {
    fn draw(&self) -> std::io::Result<Vec<crate::draw::Point<i32>>> {
        Ok(self.points.clone())
    }
}
