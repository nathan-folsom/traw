use crossterm::cursor;

use crate::draw::Draw;

pub struct Arrow {
    pub points: Vec<(i32, i32)>,
}

impl Arrow {
    pub fn update(&mut self) -> std::io::Result<()> {
        let (cursor_x, cursor_y) = cursor::position()?;
        self.points.push((cursor_x as i32, cursor_y as i32));
        Ok(())
    }
}

impl Draw for Arrow {
    fn draw(&self) -> std::io::Result<Vec<(i32, i32, char)>> {
        let mut points = vec![];

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
                points.push((
                    point.0,
                    point.1,
                    match is_vertical {
                        true => '│',
                        false => '─',
                    },
                ));
                continue;
            }

            points.push((
                point.0,
                point.1,
                get_char(&prev, &point, &self.points[i + 1]),
            ));
        }

        Ok(points)
    }
}

fn get_char(prev: &(i32, i32), current: &(i32, i32), next: &(i32, i32)) -> char {
    let positive_y = prev.1 > current.1 || next.1 > current.1;
    let negative_y = prev.1 < current.1 || next.1 < current.1;
    let positive_x = prev.0 > current.0 || next.0 > current.0;
    let negative_x = prev.0 < current.0 || next.0 < current.0;

    if negative_y && negative_x {
        '┘'
    } else if negative_y && positive_x {
        '└'
    } else if positive_x && positive_y {
        '┌'
    } else if negative_x && positive_y {
        '┐'
    } else if positive_y || negative_y {
        '│'
    } else if positive_x || negative_x {
        '─'
    } else {
        ' '
    }
}

#[cfg(test)]
mod test {
    use crate::draw::Draw;

    use super::{get_char, Arrow};

    #[test]
    fn should_get_horizontal_bar() {
        let prev = (0, 0);
        let current = (1, 0);
        let next = (2, 0);

        let next_char = get_char(&prev, &current, &next);
        assert_eq!('─', next_char);
    }

    #[test]
    fn should_get_vertical_bar() {
        let prev = (0, 0);
        let current = (0, 1);
        let next = (0, 2);

        let next_char = get_char(&prev, &current, &next);
        assert_eq!('│', next_char);
    }

    #[test]
    fn should_get_corner_1() {
        let prev = (0, 1);
        let current = (1, 1);
        let next = (1, 0);

        let next_char = get_char(&prev, &current, &next);
        assert_eq!('┘', next_char);
    }

    #[test]
    fn should_get_corner_2() {
        let prev = (0, 0);
        let current = (0, 1);
        let next = (1, 1);

        let next_char = get_char(&prev, &current, &next);
        assert_eq!('└', next_char);
    }

    #[test]
    fn should_get_corner_3() {
        let prev = (0, 1);
        let current = (0, 0);
        let next = (1, 0);

        let next_char = get_char(&prev, &current, &next);
        assert_eq!('┌', next_char);
    }

    #[test]
    fn should_get_corner_4() {
        let prev = (0, 0);
        let current = (1, 0);
        let next = (1, 1);

        let next_char = get_char(&prev, &current, &next);
        assert_eq!('┐', next_char);
    }

    #[test]
    fn should_not_render_first_position() {
        let arrow = Arrow {
            points: vec![(0, 0), (1, 0), (2, 0)],
        };

        let render = arrow.draw().unwrap();
        assert_eq!(render.len(), arrow.points.len() - 1);
    }
}
