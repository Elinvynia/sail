use crate::get_assets_folder;
use std::fmt;

// A sprite can be made from multiple textures (images in our case).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sprite {
    pub textures: Vec<TextureFile>,
}

impl Sprite {
    pub fn new(textures: Vec<TextureFile>) -> Self {
        Sprite { textures }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureFile {
    Unimplemented,
    Sea,
    Waves1,
    Waves2,
    Waves3,
    Island,
    Ship,
    Gold,
    Bananas,
    Rum,
    Water,
}

// Displaying it into a string gives us the path to it.
impl fmt::Display for TextureFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let folder = get_assets_folder();

        use TextureFile::*;
        let file = match self {
            Unimplemented => "unimplemented.png",
            Sea => "sea.png",
            Waves1 => "waves-1.png",
            Waves2 => "waves-2.png",
            Waves3 => "waves-3.png",
            Island => "island.png",
            Ship => "ship.png",
            Gold => "gold.png",
            Bananas => "bananas.png",
            Rum => "rum.png",
            Water => "water.png",
        };

        write!(f, "{}{}", folder, file)
    }
}
