use std::ops::Deref;

use super::{extension::*, AssetParser};
use crate::{error, utils::nom::*};

const COLORS_COUNT: usize = 256;
const SHADES_COUNT: usize = 32;

// TODO(nenikitov): Potentially move to a separate module
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_12_bit(color: u16) -> Self {
        // TODO(nenikitov): return result.
        assert!(color <= 0xFFF, "12 bit color is smaller than 0xFFF");

        let r = (color & 0xF00) >> 8;
        let g = (color & 0x0F0) >> 4;
        let b = color & 0x00F;

        let r = (r | r << 4) as u8;
        let g = (g | g << 4) as u8;
        let b = (b | b << 4) as u8;

        Color { r, g, b }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | self.b as u32
    }
}

impl AssetParser<Wildcard> for Color {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, color) = number::le_u32(input)?;
            Ok((input, Self::from_12_bit(color as u16)))
        }
    }
}

// TODO(Unavailable): derive ???
pub struct ColorMap {
    // TODO(nenikitov): This probably shouldn't be `pub` and should have an
    // accessor that will hide the "ugly" internal 2D-array structure.
    pub shades: Box<[[Color; COLORS_COUNT]; SHADES_COUNT]>,
}

impl AssetParser<Pack> for ColorMap {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            error::ensure_bytes_length(
                input,
                size_of::<u32>() * COLORS_COUNT * SHADES_COUNT,
                "Incorrect `ColorMap` format (256x32 array of 12-bit [padded to 32-bit] colors)",
            )?;

            let (input, colors) = multi::count!(
                |input| -> Result<[Color; COLORS_COUNT]> {
                    multi::count!(Color::parser(()))(input)
                },
                SHADES_COUNT
            )(input)?;

            let colors = {
                let colors = colors.into_boxed_slice();
                // Ensure the original box is not dropped.
                let mut colors = std::mem::ManuallyDrop::new(colors);
                // SAFETY: [_] and [_; N] has the same memory layout as long
                // as the slice contains exactly N elements.
                unsafe { Box::from_raw(colors.as_mut_ptr().cast()) }
            };

            Ok((input, Self { shades: colors }))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::LazyCell;

    use super::*;
    use crate::utils::{format::*, test::*};

    #[test]
    fn shade_works() -> eyre::Result<()> {
        assert_eq!(
            Color::parser(())(&u32::to_le_bytes(0x100))?.1,
            Color {
                r: 0x11,
                g: 0,
                b: 0
            },
        );
        assert_eq!(
            Color::parser(())(&u32::to_le_bytes(0x011))?.1,
            Color {
                r: 0,
                g: 0x11,
                b: 0x11
            },
        );
        assert_eq!(
            Color::parser(())(&u32::to_le_bytes(0x001))?.1,
            Color {
                r: 0,
                g: 0,
                b: 0x11
            },
        );
        assert_eq!(
            Color::parser(())(&u32::to_le_bytes(0x220))?.1,
            Color {
                r: 0x22,
                g: 0x22,
                b: 0x00
            },
        );
        assert_eq!(
            Color::parser(())(&u32::to_le_bytes(0x022))?.1,
            Color {
                r: 0,
                g: 0x22,
                b: 0x22
            },
        );
        assert_eq!(
            Color::parser(())(&u32::to_le_bytes(0x333))?.1,
            Color {
                r: 0x33,
                g: 0x33,
                b: 0x33
            },
        );

        Ok(())
    }

    const COLOR_MAP_DATA: LazyCell<Vec<u8>> = deflated_file!("01.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, color_map) = <ColorMap as AssetParser<Pack>>::parser(())(&COLOR_MAP_DATA)?;

        output_file(
            parsed_file_path!("color-map/monsters.png"),
            color_map.shades.as_slice().to_png(),
        )?;

        Ok(())
    }
}

pub trait PaletteTexture {
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
