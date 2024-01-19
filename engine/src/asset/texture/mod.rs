mod dat;

use self::dat::texture::TextureContext;

use super::{Asset, AssetChunk, AssetChunkWithContext, Extension, Kind};

use crate::{error, utils::nom::*};

use dat::{offset::TextureOffset, texture::MippedTexture};

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
    pub textures: Vec<MippedTexture>,
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
                MippedTexture::parse(TextureContext {
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
    use crate::{
        asset::color_map::{ColorMap, PaletteTexture},
        utils::{format::*, test::*},
    };
    use std::{cell::LazyCell, path::PathBuf};

    const COLOR_MAP_DATA: LazyCell<Vec<u8>> = deflated_file!("4F.dat");
    const TEXTURE_INFO_DATA: LazyCell<Vec<u8>> = deflated_file!("93.dat");
    const TEXTURE_DATA: LazyCell<Vec<u8>> = deflated_file!("95.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, color_map) = ColorMap::parse(&COLOR_MAP_DATA, Extension::Dat, ())?;
        let (_, offsets) = TextureOffsets::parse(&TEXTURE_INFO_DATA, Extension::Dat, ())?;
        let (_, textures) = TextureData::parse(&TEXTURE_DATA, Extension::Dat, offsets)?;

        let output_dir = PathBuf::from(parsed_file_path!("textures/"));

        textures
            .textures
            .iter()
            .enumerate()
            .try_for_each(|(i, texture)| {
                texture.mips.iter().enumerate().try_for_each(|(m, mip)| {
                    let file = output_dir.join(format!("{i:0>3X}-{m}.ppm"));

                    output_file(
                        file,
                        mip.texture.with_palette(&color_map.shades[15]).to_ppm(),
                    )
                })
            });

        Ok(())
    }
}
