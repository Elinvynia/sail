use crate::components::{Info, Money, Player, Sprite, TextureFile};
use hecs::Bundle;

pub fn player() -> impl Bundle {
    (
        Money::new(100),
        Player,
        Sprite::new(vec![TextureFile::Ship]),
        Info::new("You!", "Arrr, this is you."),
    )
}
