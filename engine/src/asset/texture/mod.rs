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
    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
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

impl AssetParser<Wildcard> for Texture {
    type Context<'ctx> = TextureSize;

    fn parser(Self::Context { width, height }: Self::Context<'_>) -> impl FnParser<Self::Output> {
        let width = width as usize;
        let height = height as usize;

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
    type Context<'ctx> = TextureOffset;

    fn parser(offset: Self::Context<'_>) -> impl FnParser<Self::Output> {
        move |input| {
            let input =
                decompress(&input[offset.offset as usize..][..offset.size_compressed as usize]);

            let (input, mip_1) = Texture::parser(TextureSize {
                width: offset.width,
                height: offset.height,
            })(&input)?;
            let (input, mip_2) = Texture::parser(TextureSize {
                width: offset.width / 2,
                height: offset.height / 2,
            })(&input)?;
            let (input, mip_3) = Texture::parser(TextureSize {
                width: offset.width / 4,
                height: offset.height / 4,
            })(&input)?;
            let (input, mip_4) = Texture::parser(TextureSize {
                width: offset.width / 8,
                height: offset.height / 8,
            })(&input)?;

            Ok((
                &[],
                Self {
                    mips: [mip_1, mip_2, mip_3, mip_4],
                },
            ))
        }
    }
}
