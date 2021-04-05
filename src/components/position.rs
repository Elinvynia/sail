#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Position {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Position { x, y, z }
    }
}
