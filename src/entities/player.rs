use crate::components::{Hoverable, Info, Money, Player, Sprite, TextureFile};
use crate::components::{Inventory, Item, ItemName::*};
use hecs::Bundle;

// Creates a player, should only be called once.
pub fn player() -> impl Bundle {
    let items = vec![Item::new(Bananas, 50), Item::new(Water, 100), Item::new(Rum, 10)];

    (
        Money::new(100),
        Inventory::new(items),
        Player,
        Sprite::new(vec![TextureFile::Ship]),
        Info::new("You!", "Arrr, this is you."),
        Hoverable,
    )
}
