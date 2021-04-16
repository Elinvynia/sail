use crate::components::TextureFile;
use hecs::World;
use std::collections::HashMap;
use tetra::graphics::{Camera, Texture};
use tetra::Context;

// Mutable global state, used in every scene.
pub struct GameWorld {
    pub ecs: World,
    pub textures: HashMap<TextureFile, Texture>,
    pub camera: Camera,
}

impl GameWorld {
    pub fn new(ctx: &mut Context) -> Self {
        GameWorld {
            ecs: World::new(),
            textures: HashMap::new(),
            camera: Camera::with_window_size(ctx),
        }
    }
}
