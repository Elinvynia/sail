use crate::components::{ItemName, TextureFile};
use std::convert::TryFrom;

// Safety wrapper for egui User textures.
// Egui stores custom textures as u64.
#[derive(Debug)]
pub enum CustomTexture {
    Unimplemented,
    Gold,
    Bananas,
    Rum,
    Water,
}

impl TryFrom<u64> for CustomTexture {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        use CustomTexture::*;
        match value {
            0 => Ok(Unimplemented),
            1 => Ok(Gold),
            2 => Ok(Bananas),
            3 => Ok(Rum),
            4 => Ok(Water),
            _ => Err(()),
        }
    }
}

impl From<CustomTexture> for u64 {
    fn from(ct: CustomTexture) -> Self {
        use CustomTexture::*;
        match ct {
            Unimplemented => 0,
            Gold => 1,
            Bananas => 2,
            Rum => 3,
            Water => 4,
        }
    }
}

impl From<CustomTexture> for TextureFile {
    fn from(ct: CustomTexture) -> Self {
        use CustomTexture::*;
        match ct {
            Unimplemented => TextureFile::Unimplemented,
            Gold => TextureFile::Gold,
            Bananas => TextureFile::Bananas,
            Rum => TextureFile::Rum,
            Water => TextureFile::Water,
        }
    }
}

impl From<ItemName> for CustomTexture {
    fn from(i: ItemName) -> Self {
        use ItemName::*;
        match i {
            Rum => CustomTexture::Rum,
            Bananas => CustomTexture::Bananas,
            Water => CustomTexture::Water,
        }
    }
}
