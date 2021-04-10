use crate::components::{Info, Sprite, TextureFile};
use hecs::Bundle;
use rand::seq::SliceRandom;
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

const ISLAND_ADJECTIVES: [&str; 1] = ["Deadly"];
const ISLAND_NOUNS: [&str; 1] = ["Outpost"];

pub fn island() -> impl Bundle {
    let mut rng = rand::thread_rng();
    let name = format!(
        "{} {}",
        ISLAND_ADJECTIVES.choose(&mut rng).unwrap(),
        ISLAND_NOUNS.choose(&mut rng).unwrap()
    );
    (
        Info::new(&name, "You can trade here!"),
        Sprite {
            textures: vec![TextureFile::Island],
        },
    )
}
