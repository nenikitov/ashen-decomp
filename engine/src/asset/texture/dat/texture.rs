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

pub struct TextureContext<'a> {
    pub full_data: &'a [u8],
    pub offset: TextureOffset,
}

impl AssetChunkWithContext for MippedTexture {
    type Context<'a> = TextureContext<'a>;

    fn parse(context: Self::Context<'_>) -> impl Fn(&[u8]) -> Result<Self> {
        let texture_data = decompress(
            &context.full_data[context.offset.offset as usize..]
                [..context.offset.size_compressed as usize],
        );
        move |input| {
            macro_rules! mip {
                ($mip: literal) => {{
                    Texture::parse((
                        context.offset.width / 2u16.pow($mip),
                        context.offset.height / 2u16.pow($mip))
                    )(&texture_data)
                }};
            }

            let (input, mip_1) = mip!(0)?;
            let (input, mip_2) = mip!(1)?;
            let (input, mip_3) = mip!(2)?;
            let (input, mip_4) = mip!(3)?;

            Ok((
                &[],
                Self {
                    mips: [mip_1, mip_2, mip_3, mip_4],
                },
            ))
        }
    }
}
