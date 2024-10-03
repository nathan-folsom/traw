use crate::cursor::adjust_position;

pub struct MotionState {
    count: Vec<char>,
}

impl MotionState {
    pub fn new() -> Self {
        Self { count: vec![] }
    }
    pub fn handle_motions(&mut self, key: char) -> std::io::Result<()> {
        let move_count = self.get_count() as i16;
        match key {
            'h' => {
                adjust_position(-move_count, 0);
            }
            'j' => {
                adjust_position(0, move_count);
            }
            'k' => {
                adjust_position(0, -move_count);
            }
            'l' => {
                adjust_position(move_count, 0);
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
