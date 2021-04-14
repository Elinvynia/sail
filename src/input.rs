use tetra::input::Key;

#[derive(Debug)]
pub enum CameraDir {
    Up,
    Down,
    Left,
    Right,
}

pub fn key_to_cameradir(key: &Key) -> Option<CameraDir> {
    match key {
        Key::Up => Some(CameraDir::Up),
        Key::Down => Some(CameraDir::Down),
        Key::Left => Some(CameraDir::Left),
        Key::Right => Some(CameraDir::Right),
        _ => None,
    }
}

#[derive(Debug)]
pub enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

pub fn key_to_movedir(key: &Key) -> Option<MoveDir> {
    match key {
        Key::W => Some(MoveDir::Up),
        Key::S => Some(MoveDir::Down),
        Key::A => Some(MoveDir::Left),
        Key::D => Some(MoveDir::Right),
        _ => None,
    }
}
