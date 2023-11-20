use super::{Asset, Extension, Kind};
use crate::{error, utils::nom::*};
use std::mem;

const COLORS_COUNT: usize = 256;
const SHADES_COUNT: usize = 32;

// TODO(nenikitov): Potentially move to a separate module
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// TODO(Unavailable): derive
pub struct ColorMap {
    // TODO(nenikitov): This probably shouldn't be `pub` and should have an accessor that will hide the "ugly" internal 2D-array structure
    pub shades: Box<[[Color; COLORS_COUNT]; SHADES_COUNT]>,
}

#[allow(clippy::cast_possible_truncation)]
// TODO(Unavailable): change name. `12_bit_color_from_u32` would be more descriptive.
fn shade(input: &[u8]) -> Result<Color> {
    let (input, color) = number::le_u32(input)?;
    // TODO(Unavailable): verify with nom
    assert!(color <= 0xFFF, "12 bit color is smaller than 0xFFF");

    let r = (color & 0xF00) >> 8;
    let g = (color & 0x0F0) >> 4;
    let b = color & 0x00F;

    let r = (r | r << 4) as u8;
    let g = (g | g << 4) as u8;
    let b = (b | b << 4) as u8;

    Ok((input, Color { r, g, b }))
}

impl Asset for ColorMap {
    fn kind() -> Kind {
        Kind::ColorMap
    }

    fn parse(input: &[u8], extension: Extension) -> Result<Self> {
        fn colors(input: &[u8]) -> Result<[Color; COLORS_COUNT]> {
            multi::count_const::<COLORS_COUNT, _, _, _>(shade)(input)
        }

        if extension != Extension::Dat {
            return Err(error::ParseError::unsupported_extension(input, extension).into());
        }

        // NOTE: These checks depend on which `extension` is being parsed.
        error::ensure_bytes_length(
            input,
            mem::size_of::<u32>() * COLORS_COUNT * SHADES_COUNT,
            "Incorrect `ColorMap` format (256x32 array of 12-bit [padded to 32-bit] colors)",
        )?;

        let (input, colors) = multi::count_const::<SHADES_COUNT, _, _, _>(colors)(input)?;

        Ok((
            input,
            Self {
                shades: Box::new(colors),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shade_works() -> eyre::Result<()> {
        assert_eq!(
            shade(&u32::to_le_bytes(0x100))?.1,
            Color {
                r: 0x11,
                g: 0,
                b: 0
            },
        );
        assert_eq!(
            shade(&u32::to_le_bytes(0x011))?.1,
            Color {
                r: 0,
                g: 0x11,
                b: 0x11
            },
        );
        assert_eq!(
            shade(&u32::to_le_bytes(0x001))?.1,
            Color {
                r: 0,
                g: 0,
                b: 0x11
            },
        );
        assert_eq!(
            shade(&u32::to_le_bytes(0x220))?.1,
            Color {
                r: 0x22,
                g: 0x22,
                b: 0x00
            },
        );
        assert_eq!(
            shade(&u32::to_le_bytes(0x022))?.1,
            Color {
                r: 0,
                g: 0x22,
                b: 0x22
            },
        );
        assert_eq!(
            shade(&u32::to_le_bytes(0x333))?.1,
            Color {
                r: 0x33,
                g: 0x33,
                b: 0x33
            },
        );

        Ok(())
    }
}
