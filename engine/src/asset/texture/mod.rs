mod dat;

use super::{extension::*, AssetParser};
use crate::utils::{compression::decompress, nom::*};
use dat::{offset::TextureOffset, size::TextureSize, texture::MippedTexture};

pub struct TextureOffsetCollection;

impl AssetParser<Pack> for TextureOffsetCollection {
    type Output = Vec<TextureOffset>;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (_, offsets) = multi::many0(TextureOffset::parser(()))(input)?;

            Ok((&[], offsets))
        }
    }
}

pub struct MippedTextureCollection;

impl AssetParser<Pack> for MippedTextureCollection {
    type Output = Vec<MippedTexture>;

    type Context<'ctx> = &'ctx [TextureOffset];

    fn parser(offsets: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let textures = offsets
                .iter()
                .map(|o| {
                    let input = &input[o.offset as usize..][..o.size_compressed as usize];
                    let input = decompress(input);

                    MippedTexture::parser(TextureSize {
                        width: o.width,
                        height: o.height,
                    })(&input)
                    .map(|(_, d)| d)
                })
                .collect::<std::result::Result<Vec<_>, _>>()?;

            Ok((&[], textures))
        }
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
        let (_, color_map) = <ColorMap as AssetParser<Pack>>::parser(())(&COLOR_MAP_DATA)?;
        let (_, offsets) =
            <TextureOffsetCollection as AssetParser<Pack>>::parser(())(&TEXTURE_INFO_DATA)?;
        let (_, textures) =
            <MippedTextureCollection as AssetParser<Pack>>::parser(&offsets)(&TEXTURE_DATA)?;

        let output_dir = PathBuf::from(parsed_file_path!("textures/"));

        textures.iter().enumerate().try_for_each(|(i, texture)| {
            texture.mips.iter().enumerate().try_for_each(|(m, mip)| {
                let file = output_dir.join(format!("{i:0>3X}-{m}.png"));

                output_file(
                    file,
                    mip.colors.with_palette(&color_map.shades[15]).to_png(),
                )
            })
        });

        Ok(())
    }
}
