use hecs::World;
use std::collections::HashMap;
use tetra::graphics::Texture;

// Mutable global state, used in every scene.
#[derive(Default)]
pub struct GameWorld {
    pub ecs: World,
    pub textures: HashMap<String, Texture>,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld::default()
    }
}
