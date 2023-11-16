use super::{Asset, Extension, Kind};

const COLORS_COUNT: usize = 256;
const SHADES_COUNT: usize = 32;

// TODO(nenikitov): Potentially move to a separate module
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct ColorMap {
    // TODO(nenikitov): This probably shouldn't be `pub` and should have an accessor that will hide the "ugly" internal 2D-array structure
    pub shades: Box<[[u8; COLORS_COUNT]; SHADES_COUNT]>,
}

impl Asset for ColorMap {
    fn kind() -> Kind {
        Kind::ColorMap
    }

    fn parse(bytes: &[u8], extension: Extension) -> Self {
        // TODO(nenikitov): figure out how to import `nom` and fix compilation issues
        fn colors(input: &[u8]) -> Option<[Color; COLORS_COUNT]> {
            fn shade(input: &[u8]) -> Option<Color> {
                let color = number::le_u32(input)?;
                if color >= 0xFFF {
                    return None;
                }

                let r = (color & 0xF00) >> 8;
                let g = (color & 0x0F0) >> 4;
                let b = color & 0x00F;

                let r = (r | r << 4) as u8;
                let g = (g | g << 4) as u8;
                let b = (b | b << 4) as u8;

                Some(Color { r, g, b })
            }

            multi::count(shade, COLORS_COUNT)(input)
        }

        assert!(extension == Extension::Dat);
        // TODO(nenikitov): Remove `expect()` when parsing changes to return errors
        multi::count(colors, SHADES_COUNT)(input).expect("Color map is of correct format (256x32 array of 12-bit colors)")
    }
}
