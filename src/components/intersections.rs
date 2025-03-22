use crate::{
    characters::{INTERSECTION_DOWN, INTERSECTION_LEFT, INTERSECTION_RIGHT, INTERSECTION_UP},
    draw::{Color, CursorIntersect, Draw, Intersection, Point},
    mode::Mode,
    shape::Shape,
    state::State,
    util::Vec2,
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
            |point: Option<&Vec2<i32>>, reference: Option<&Vec2<i32>>| {
                if let Some(p) = point {
                    all_rectangles.iter().for_each(|r| {
                        let intersection = r.get_intersection(p);
                        if let Intersection::Edge(_) = intersection {
                            if let Some(r) = reference {
                                let character = {
                                    if r.x > p.x {
                                        INTERSECTION_RIGHT
                                    } else if r.x < p.x {
                                        INTERSECTION_LEFT
                                    } else if r.y < p.y {
                                        INTERSECTION_UP
                                    } else if r.y > p.y {
                                        INTERSECTION_DOWN
                                    } else {
                                        unreachable!("Reference point should always be different than endpoint")
                                    }
                                };
                                intersection_points.push(Point {
                                    x: p.x,
                                    y: p.y,
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
