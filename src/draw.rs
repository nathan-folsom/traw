pub trait Draw {
    fn draw(&self) -> std::io::Result<()>;
}

pub struct Renderer {
    x: u32,
    y: u32,
    width: u16,
    height: u16,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    pub fn render(&self, shape: &impl Draw) -> std::io::Result<()> {
        shape.draw()?;

        Ok(())
    }
}
