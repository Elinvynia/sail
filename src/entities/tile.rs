use crate::components::{Sprite, TextureFile};
use hecs::Bundle;
use rand::Rng;

pub fn sea() -> impl Bundle {
    let waves = match rand::thread_rng().gen_range(1..=3) {
        1 => TextureFile::Waves1,
        2 => TextureFile::Waves2,
        3 => TextureFile::Waves3,
        _ => unreachable!(),
    };

    (Sprite {
        textures: vec![TextureFile::Sea, waves],
    },)
}
