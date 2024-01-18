mod dat;

use self::dat::texture::TextureContext;

use super::{Asset, AssetChunk, AssetChunkWithContext, Extension, Kind};

use crate::{error, utils::nom::*};

use dat::{offset::TextureOffset, texture::Texture};

pub struct TextureOffsets {
    offsets: Vec<TextureOffset>,
}

impl Asset for TextureOffsets {
    type Context = ();

    fn kind() -> Kind {
        Kind::TextureInfo
    }

    fn parse(input: &[u8], extension: Extension, _: Self::Context) -> Result<Self> {
        match extension {
            Extension::Dat => {
                let (_, offsets) = multi::many0(TextureOffset::parse)(input)?;

                Ok((&[], Self { offsets }))
            }
            _ => Err(error::ParseError::unsupported_extension(input, extension).into()),
        }
    }
}

pub struct TextureData {
    pub textures: Vec<Texture>,
}

impl Asset for TextureData {
    type Context = TextureOffsets;

    fn kind() -> Kind {
        Kind::TextureData
    }

    fn parse(input: &[u8], extension: Extension, offsets: Self::Context) -> Result<Self> {
        let textures = offsets
            .offsets
            .into_iter()
            .map(|o| {
                Texture::parse(TextureContext {
                    full_data: input,
                    offset: o,
                })(&input)
                .map(|(_, d)| d)
            })
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok((&[], Self { textures }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{format::*, test::*};
    use std::{cell::LazyCell, path::PathBuf};

    const TEXTURE_INFO: LazyCell<Vec<u8>> = deflated_file!("93.dat");
    const TEXTURE_DATA: LazyCell<Vec<u8>> = deflated_file!("95.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        // let (_, offsets) = TextureOffsets::parse(&TEXTURE_INFO, Extension::Dat, ())?;
        // let (_, textures) = TextureData::parse(&TEXTURE_DATA, Extension::Dat, offsets)?;

        let output_dir = PathBuf::from(parsed_file_path!("textures/"));

        Ok(())
    }
}
