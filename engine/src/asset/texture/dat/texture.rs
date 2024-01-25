use super::size::TextureSize;
use crate::{
    asset::{extension::*, AssetParser},
    utils::nom::*,
};
use itertools::Itertools;

pub struct Texture {
    pub colors: Vec<Vec<u8>>,
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
