use crate::components::{Info, Island, Sea, Sprite, TextureFile};
use hecs::Bundle;
use rand::seq::SliceRandom;
use rand::Rng;

// Creates a sea tile with randomized waves.
pub fn sea() -> impl Bundle {
    let waves = match rand::thread_rng().gen_range(1..=3) {
        1 => TextureFile::Waves1,
        2 => TextureFile::Waves2,
        3 => TextureFile::Waves3,
        _ => unreachable!(),
    };

    (Sprite::new(vec![TextureFile::Sea, waves]), Sea)
}

const ISLAND_ADJECTIVES: [&str; 1] = ["Deadly"];
const ISLAND_NOUNS: [&str; 1] = ["Outpost"];

// Creates an island with randomized name and (soon) randomized loot.
pub fn island() -> impl Bundle {
    let mut rng = rand::thread_rng();
    let name = format!(
        "{} {}",
        ISLAND_ADJECTIVES.choose(&mut rng).unwrap(),
        ISLAND_NOUNS.choose(&mut rng).unwrap()
    );

    (
        Info::new(&name, "You can trade here!"),
        Sprite::new(vec![TextureFile::Island]),
        Island,
    )
}
