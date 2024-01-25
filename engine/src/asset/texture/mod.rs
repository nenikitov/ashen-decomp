use std::ops::Div;

use super::{extension::*, AssetParser};
use crate::utils::{compression::decompress, nom::*};
use itertools::Itertools;

pub struct TextureOffset {
    pub width: u16,
    pub height: u16,
    pub offset: u32,
    pub size_compressed: u32,
    pub size_decompressed: u32,
    pub animation_frames: u32,
    pub next_animation_texture_id: u32,
}

impl AssetParser<Wildcard> for TextureOffset {
    type Context<'ctx> = ();

    type Output = Self;

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, width) = number::le_u16(input)?;
            let (input, height) = number::le_u16(input)?;
            let (input, offset) = number::le_u32(input)?;
            let (input, size_compressed) = number::le_u32(input)?;
            let (input, size_decompressed) = number::le_u32(input)?;
            let (input, animation_frames) = number::le_u32(input)?;
            let (input, next_animation_texture_id) = number::le_u32(input)?;

            Ok((
                input,
                Self {
                    width,
                    height,
                    offset,
                    size_compressed,
                    size_decompressed,
                    animation_frames,
                    next_animation_texture_id,
                },
            ))
        }
    }
}

pub struct Texture {
    pub colors: Vec<Vec<u8>>,
}

pub struct TextureSize {
    width: u16,
    height: u16,
}

impl Div<u16> for TextureSize {
    type Output = Self;

    fn div(self, rhs: u16) -> Self::Output {
        &self / rhs
    }
}

impl Div<u16> for &TextureSize {
    type Output = TextureSize;

    fn div(self, rhs: u16) -> Self::Output {
        TextureSize {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}

impl AssetParser<Wildcard> for Texture {
    type Output = Self;

    type Context<'ctx> = &'ctx TextureSize;

    fn parser(size: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        let width = size.width as usize;
        let height = size.height as usize;

        move |input| {
            let (input, colors) = multi::count!(number::le_u8, width * height)(input)?;

            let colors = colors
                .into_iter()
                .chunks(width)
                .into_iter()
                .map(Iterator::collect)
                .collect();

            Ok((input, Self { colors }))
        }
    }
}

pub struct MippedTexture {
    pub mips: [Texture; 4],
}

impl AssetParser<Wildcard> for MippedTexture {
    type Output = Self;

    type Context<'ctx> = TextureSize;

    fn parser(size: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, mip_1) = Texture::parser(&size)(&input)?;
            let (input, mip_2) = Texture::parser(&(&size / 2))(&input)?;
            let (input, mip_3) = Texture::parser(&(&size / 4))(&input)?;
            let (input, mip_4) = Texture::parser(&(&size / 8))(&input)?;

            Ok((
                &[],
                Self {
                    mips: [mip_1, mip_2, mip_3, mip_4],
                },
            ))
        }
    }
}

pub struct TextureOffsetCollection {}

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

pub struct MippedTextureCollection {}

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
