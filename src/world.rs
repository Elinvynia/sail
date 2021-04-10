use crate::components::TextureFile;
use hecs::World;
use std::collections::HashMap;
use tetra::graphics::Texture;
use tetra::graphics::mesh::Mesh;

// Mutable global state, used in every scene.
#[derive(Default)]
pub struct GameWorld {
    pub ecs: World,
    pub textures: HashMap<TextureFile, Texture>,
    pub egui_cache: HashMap<String, Mesh>,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld::default()
    }
}
