use crate::components::TextureFile;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum CustomTexture {
    Gold,
}

impl TryFrom<u64> for CustomTexture {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        use CustomTexture::*;
        match value {
            1 => Ok(Gold),
            _ => Err(()),
        }
    }
}

impl From<CustomTexture> for u64 {
    fn from(ct: CustomTexture) -> Self {
        use CustomTexture::*;
        match ct {
            Gold => 1,
        }
    }
}

impl From<CustomTexture> for TextureFile {
    fn from(ct: CustomTexture) -> Self {
        use CustomTexture::*;
        match ct {
            Gold => TextureFile::Gold,
        }
    }
}
