use itertools::Itertools;

use crate::{asset::AssetChunkWithContext, utils::compression::decompress, utils::nom::*};

use super::offset::TextureOffset;

pub struct Texture {
    pub texture: Vec<Vec<u8>>,
}

pub struct MippedTexture {
    pub mips: [Texture; 4],
}

impl AssetChunkWithContext for Texture {
    type Context<'a> = (u16, u16);

    fn parse((width, height): Self::Context<'_>) -> impl Fn(&[u8]) -> Result<Self> {
        move |input| {
            let width = width as usize;
            let height = height as usize;

            let (input, texture) = multi::count!(number::le_u8, width * height)(input)?;
            let texture = texture
                .into_iter()
                .chunks(width)
                .into_iter()
                .map(Iterator::collect)
                .collect();

            Ok((input, Self { texture }))
        }
    }
}

impl AssetChunkWithContext for MippedTexture {
    type Context<'a> = &'a TextureOffset;

    fn parse(context: Self::Context<'_>) -> impl Fn(&[u8]) -> Result<Self> {
        move |input| {
            let input =
                decompress(&input[context.offset as usize..][..context.size_compressed as usize]);

            let (input, mip_1) = Texture::parse((context.width, context.height))(&input)?;
            let (input, mip_2) = Texture::parse((context.width / 2, context.height / 2))(&input)?;
            let (input, mip_3) = Texture::parse((context.width / 4, context.height / 4))(&input)?;
            let (input, mip_4) = Texture::parse((context.width / 8, context.height / 8))(&input)?;

            Ok((
                &[],
                Self {
                    mips: [mip_1, mip_2, mip_3, mip_4],
                },
            ))
        }
    }
}
