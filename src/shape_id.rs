use std::sync::{OnceLock, RwLock};

static SHAPE_ID_STORE: OnceLock<RwLock<ShapeIdStore>> = OnceLock::new();

struct ShapeIdStore {
    current: u32,
}

impl ShapeIdStore {
    fn generate(&mut self) -> u32 {
        let next = self.current + 1;
        self.current = next;
        next
    }
}

pub fn generate_shape_id() -> u32 {
    SHAPE_ID_STORE
        .get_or_init(|| RwLock::new(ShapeIdStore { current: 0 }))
        .write()
        .unwrap()
        .generate()
}
