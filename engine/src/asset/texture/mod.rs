mod dat;

use crate::{error, utils::nom::*};

use dat::offset::TextureOffset;

use super::{Asset, AssetChunk, Extension, Kind};

struct TextureInfo {
    offsets: Vec<TextureOffset>,
}

impl Asset for TextureInfo {
    fn kind() -> Kind {
        Kind::TextureInfo
    }

    fn parse(input: &[u8], extension: Extension) -> Result<Self> {
        match extension {
            Extension::Dat => {
                let (_, offsets) = multi::many1(TextureOffset::parse)(input)?;

                Ok((&[], Self { offsets }))
            }
            _ => Err(error::ParseError::unsupported_extension(input, extension).into()),
        }
    }
}
