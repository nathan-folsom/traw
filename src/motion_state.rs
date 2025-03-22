use std::cmp::Ordering;

use crate::{
    cursor::{adjust_position, cursor_position, set_position},
    mode::Mode,
    renderer::Renderer,
    util::Vec2,
};

pub struct MotionState {
    count: Vec<char>,
}

impl MotionState {
    pub fn new() -> Self {
        Self { count: vec![] }
    }

    pub fn handle_motions(
        &mut self,
        key: char,
        renderer: &Renderer,
        mode: &Mode,
    ) -> std::io::Result<()> {
        let move_count = self.get_count() as i16;
        match key {
            'h' => {
                adjust_position((-move_count, 0).into());
            }
            'j' => {
                adjust_position((0, move_count).into());
            }
            'k' => {
                adjust_position((0, -move_count).into());
            }
            'l' => {
                adjust_position((move_count, 0).into());
            }
            'w' => {
                if mode.is_normal() {
                    word_motion(renderer, get_next_word_start);
                }
            }
            'b' => {
                if mode.is_normal() {
                    word_motion(renderer, get_previous_word_start);
                }
            }
            'e' => {
                if mode.is_normal() {
                    word_motion(renderer, get_next_word_end);
                }
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.count.push(key),
            _ => {}
        }

        Ok(())
    }

    fn get_count(&mut self) -> u16 {
        let chars = std::mem::take(&mut self.count);
        chars.iter().collect::<String>().parse::<u16>().unwrap_or(1)
    }
}

type Position = (Vec2<u16>, Option<u32>);
type ShapePositions = Vec<Position>;
type JumpPosition = Option<Vec2<u16>>;

fn word_motion<T>(renderer: &Renderer, position_getter: T)
where
    T: Fn(Vec2<u16>, ShapePositions) -> JumpPosition,
{
    let current_position = cursor_position();
    let points = renderer
        .state
        .iter()
        .enumerate()
        .flat_map(|(x, col)| {
            col.iter()
                .enumerate()
                .map(|(y, point)| (Vec2::new(x as u16, y as u16), point.shape_id))
                .collect::<ShapePositions>()
        })
        .collect();
    let next_position = position_getter(current_position, points);

    if let Some(next) = next_position {
        set_position(next);
    }
}

fn get_next_word_start(cursor_position: Vec2<u16>, points: ShapePositions) -> JumpPosition {
    let mut points = points;
    points.sort();

    let jump_to = points.iter().enumerate().find(|(i, (position, shape_id))| {
        Ordering::is_gt(position.cmp(&cursor_position))
            && shape_id.map_or(false, |id| {
                i.checked_sub(1).map_or(false, |i| {
                    points
                        .get(i)
                        .and_then(|(_, id)| *id)
                        .map_or(true, |n| n != id)
                })
            })
    });

    jump_to.map(|(_, (point, _))| point.clone())
}

fn get_previous_word_start(cursor_position: Vec2<u16>, points: ShapePositions) -> JumpPosition {
    let mut points = points;
    points.sort_by(|a, b| a.cmp(b).reverse());

    let jump_to = points.iter().enumerate().find(|(i, (position, shape_id))| {
        Ordering::is_lt(position.cmp(&cursor_position))
            && shape_id.map_or(false, |id| {
                points
                    .get(i + 1)
                    .and_then(|(_, id)| *id)
                    .map_or(true, |n| n != id)
            })
    });

    jump_to.map(|(_, (point, _))| point.clone())
}

fn get_next_word_end(cursor_position: Vec2<u16>, points: ShapePositions) -> JumpPosition {
    let mut points = points;
    points.sort();

    let jump_to = points.iter().enumerate().find(|(i, (position, shape_id))| {
        Ordering::is_gt(position.cmp(&cursor_position))
            && shape_id.map_or(false, |id| {
                points
                    .get(i + 1)
                    .and_then(|(_, id)| *id)
                    .map_or(true, |n| n != id)
            })
    });

    jump_to.map(|(_, (point, _))| point.clone())
}

#[cfg(test)]
mod next_word_start_tests {
    use super::{get_next_word_start, Vec2};

    #[test]
    fn should_get_next_in_row() {
        let current_position = Vec2::new(0, 0);
        let points = vec![(current_position.clone(), None), (Vec2::new(1, 0), Some(1))];
        let result = get_next_word_start(current_position, points);
        assert!(result.is_some());
        assert!(result.as_ref().unwrap().x == 1);
        assert!(result.unwrap().y == 0);
    }

    #[test]
    fn should_get_next_shape_id() {
        let current_position = Vec2::new(0, 0);
        let points = vec![
            (current_position.clone(), Some(0)),
            (Vec2::new(1, 0), Some(0)),
            (Vec2::new(2, 0), Some(1)),
        ];
        let result = get_next_word_start(current_position, points);
        assert!(result.is_some());
        assert!(result.as_ref().unwrap().x == 2);
        assert!(result.unwrap().y == 0);
    }

    #[test]
    fn should_get_next_row_if_none_in_row() {
        let current_position = Vec2::new(0, 0);
        let points = vec![
            (current_position.clone(), Some(0)),
            (Vec2::new(1, 0), Some(0)),
            (Vec2::new(0, 1), Some(1)),
        ];
        let result = get_next_word_start(current_position, points);
        assert!(result.is_some());
        assert!(result.as_ref().unwrap().x == 0);
        assert!(result.unwrap().y == 1);
    }

    #[test]
    fn should_search_by_row() {
        let current_position = Vec2::new(0, 0);
        let points = vec![
            (current_position.clone(), Some(0)),
            (Vec2::new(0, 1), Some(1)),
            (Vec2::new(1, 0), Some(1)),
        ];
        let result = get_next_word_start(current_position, points);
        assert!(result.is_some());
        assert!(
            result.as_ref().unwrap().x == 1,
            "Expected {} to equal 1",
            result.as_ref().unwrap().x
        );
        assert!(result.unwrap().y == 0);
    }

    #[test]
    fn should_get_row_in_order() {
        let current_position = Vec2::new(0, 0);
        let points = vec![
            (current_position.clone(), Some(0)),
            (Vec2::new(2, 0), Some(2)),
            (Vec2::new(1, 0), Some(1)),
        ];
        let result = get_next_word_start(current_position, points);
        assert!(result.is_some());
        assert!(
            result.as_ref().unwrap().x == 1,
            "Expected {} to equal 1",
            result.as_ref().unwrap().x
        );
        assert!(result.unwrap().y == 0);
    }

    #[test]
    fn should_get_same_word_next_line() {
        let current_position = Vec2::new(0, 0);
        let points = vec![
            (current_position.clone(), Some(0)),
            (Vec2::new(1, 0), None),
            (Vec2::new(0, 1), Some(0)),
        ];
        let result = get_next_word_start(current_position, points);
        assert!(result.is_some());
        assert!(
            result.as_ref().unwrap().x == 0,
            "Expected {} to equal 0",
            result.as_ref().unwrap().x
        );
        assert!(result.unwrap().y == 1);
    }
}

#[cfg(test)]
mod previous_word_start_tests {
    use super::{get_previous_word_start, Vec2};

    #[test]
    fn should_get_prev_in_row() {
        let current_position = Vec2::new(1, 0);
        let points = vec![(Vec2::new(0, 0), Some(1)), (current_position.clone(), None)];
        let result = get_previous_word_start(current_position, points);
        assert!(result.is_some());
        assert!(result.as_ref().unwrap().x == 0);
        assert!(result.unwrap().y == 0);
    }

    #[test]
    fn should_go_to_beginning_of_previous() {
        let current_position = Vec2::new(2, 0);
        let points = vec![
            (Vec2::new(0, 0), Some(1)),
            (Vec2::new(1, 0), Some(1)),
            (current_position.clone(), None),
        ];
        let result = get_previous_word_start(current_position, points);
        assert!(result.is_some());
        assert!(result.as_ref().unwrap().x == 0);
        assert!(result.unwrap().y == 0);
    }

    #[test]
    fn should_get_prev_row_if_none_in_row() {
        let current_position = Vec2::new(1, 1);
        let points = vec![
            (Vec2::new(0, 0), Some(0)),
            (Vec2::new(1, 0), Some(0)),
            (Vec2::new(0, 1), None),
            (current_position.clone(), Some(1)),
        ];
        let result = get_previous_word_start(current_position, points);
        assert!(result.is_some());
        assert!(result.as_ref().unwrap().x == 0);
        assert!(result.unwrap().y == 0);
    }

    #[test]
    fn should_search_by_row() {
        let current_position = Vec2::new(2, 1);
        let points = vec![
            (Vec2::new(0, 0), None),
            (Vec2::new(1, 0), None),
            (Vec2::new(2, 0), Some(1)),
            (Vec2::new(0, 1), None),
            (Vec2::new(1, 1), Some(1)),
            (current_position.clone(), Some(1)),
        ];
        let result = get_previous_word_start(current_position, points);
        assert!(result.is_some());
        assert!(
            result.as_ref().unwrap().x == 1,
            "Expected {} to equal 1",
            result.as_ref().unwrap().x
        );
        assert!(result.unwrap().y == 1);
    }

    #[test]
    fn should_get_row_in_order() {
        let current_position = Vec2::new(2, 0);
        let points = vec![
            (Vec2::new(0, 0), Some(0)),
            (current_position.clone(), Some(2)),
            (Vec2::new(1, 0), Some(1)),
        ];
        let result = get_previous_word_start(current_position, points);
        assert!(result.is_some());
        assert!(
            result.as_ref().unwrap().x == 1,
            "Expected {} to equal 1",
            result.as_ref().unwrap().x
        );
        assert!(result.unwrap().y == 0);
    }
}

#[cfg(test)]
mod get_next_word_end_tests {
    use crate::{motion_state::get_next_word_end, util::Vec2};

    #[test]
    fn should_get_end_of_next_word() {
        let current_position = Vec2::new(0, 0);
        let points = vec![
            (current_position.clone(), Some(0)),
            (Vec2::new(1, 0), Some(1)),
            (Vec2::new(2, 0), Some(1)),
        ];
        let result = get_next_word_end(current_position, points);
        assert!(result.is_some());
        assert!(
            result.as_ref().unwrap().x == 2,
            "Expected {} to equal 2",
            result.as_ref().unwrap().x
        );
        assert!(result.unwrap().y == 0);
    }

    #[test]
    fn should_get_end_of_current_word() {
        let current_position = Vec2::new(0, 0);
        let points = vec![
            (current_position.clone(), Some(0)),
            (Vec2::new(1, 0), Some(0)),
            (Vec2::new(2, 0), Some(1)),
        ];
        let result = get_next_word_end(current_position, points);
        assert!(result.is_some());
        assert!(
            result.as_ref().unwrap().x == 1,
            "Expected {} to equal 2",
            result.as_ref().unwrap().x
        );
        assert!(result.unwrap().y == 0);
    }

    #[test]
    fn should_stay_on_current_line() {
        let current_position = Vec2::new(0, 0);
        let points = vec![
            (current_position.clone(), Some(0)),
            (Vec2::new(1, 0), Some(0)),
            (Vec2::new(2, 0), Some(1)),
            (Vec2::new(0, 1), Some(0)),
            (Vec2::new(1, 1), Some(0)),
            (Vec2::new(2, 1), Some(1)),
        ];
        let result = get_next_word_end(current_position, points);
        assert!(result.is_some());
        assert!(
            result.as_ref().unwrap().x == 1,
            "Expected {} to equal 2",
            result.as_ref().unwrap().x
        );
        assert!(result.unwrap().y == 0);
    }

    #[test]
    fn should_jump_to_end_of_first_shape() {
        let current_position = Vec2::new(0, 0);
        let points = vec![
            (current_position.clone(), None),
            (Vec2::new(1, 0), None),
            (Vec2::new(2, 0), Some(1)),
            (Vec2::new(3, 0), Some(1)),
        ];
        let result = get_next_word_end(current_position, points);
        assert!(result.is_some());
        assert!(
            result.as_ref().unwrap().x == 3,
            "Expected {} to equal 2",
            result.as_ref().unwrap().x
        );
        assert!(result.unwrap().y == 0);
    }
}
