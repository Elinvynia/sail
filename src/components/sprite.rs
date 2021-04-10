use crate::get_assets_folder;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sprite {
    pub textures: Vec<TextureFile>,
}

impl Sprite {
    pub fn new(textures: Vec<TextureFile>) -> Self {
        Sprite { textures }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureFile {
    Player,
    Sea,
    Unimplemented,
    Waves1,
    Waves2,
    Waves3,
    Island,
}

impl fmt::Display for TextureFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let folder = get_assets_folder();

        use TextureFile::*;
        let file = match self {
            Player => "player.png",
            Sea => "sea.png",
            Unimplemented => "unimplemented.png",
            Waves1 => "waves-1.png",
            Waves2 => "waves-2.png",
            Waves3 => "waves-3.png",
            Island => "island.png",
        };

        write!(f, "{}{}", folder, file)
    }
}
