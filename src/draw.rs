pub trait Draw {
    fn draw(&self) -> std::io::Result<()>;
}
