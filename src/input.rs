use tetra::input::Key;

#[derive(Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn key_to_dir(key: &Key) -> Option<Dir> {
    match key {
        Key::W => Some(Dir::Up),
        Key::S => Some(Dir::Down),
        Key::A => Some(Dir::Left),
        Key::D => Some(Dir::Right),
        _ => None,
    }
}
