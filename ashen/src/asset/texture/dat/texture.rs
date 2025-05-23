use std::ops::Deref;

use itertools::Itertools;

use super::size::TextureSize;
use crate::{
    asset::{Parser, color_map::Color},
    utils::nom::*,
};

// TODO(nenikitov): Move this to a separate public module later
#[derive(Clone)]
pub struct Texture {
    pub colors: Vec<Vec<u8>>,
}

impl Parser for Texture {
    type Context<'ctx> = TextureSize;

    fn parser(size: Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
        move |input| {
            let (input, colors) = multi::count!(number::le_u8, size.width * size.height)(input)?;

            let colors = colors
                .into_iter()
                .chunks(size.width)
                .into_iter()
                .map(Iterator::collect)
                .collect();

            Ok((input, Self { colors }))
        }
    }
}

impl Texture {
    pub fn width(&self) -> usize {
        self.colors[0].len()
    }

    pub fn height(&self) -> usize {
        self.colors.len()
    }
}

#[derive(Clone)]
pub struct MippedTexture {
    pub mips: [Texture; 4],
}

impl Parser for MippedTexture {
    type Context<'ctx> = TextureSize;

    fn parser(size: Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
        move |input| {
            let (input, mip_1) = Texture::parser(size)(input)?;
            let (input, mip_2) = Texture::parser(size / 2)(input)?;
            let (input, mip_3) = Texture::parser(size / 4)(input)?;
            let (input, mip_4) = Texture::parser(size / 8)(input)?;

            Ok((
                &[],
                Self {
                    mips: [mip_1, mip_2, mip_3, mip_4],
                },
            ))
        }
    }
}

pub trait PaletteTexture {
    // TODO(Unavailable): `&[Color; 256]`
    fn with_palette(&self, palette: &[Color]) -> Vec<Vec<Color>>;
}

// impl for any 2D array like data structure.
impl<Outer: ?Sized, Inner> PaletteTexture for Outer
where
    Outer: Deref<Target = [Inner]>,
    Inner: AsRef<[u8]>,
{
    fn with_palette(&self, palette: &[Color]) -> Vec<Vec<Color>> {
        self.iter()
            .map(|c| c.as_ref().iter().map(|c| palette[*c as usize]).collect())
            .collect()
    }
}

impl PaletteTexture for Texture {
    fn with_palette(&self, palette: &[Color]) -> Vec<Vec<Color>> {
        self.colors.with_palette(palette)
    }
}
