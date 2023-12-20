use super::{Asset, AssetChunk, Extension, Kind};
use crate::{asset::color_map::Color, error, utils::nom::*};
use itertools::Itertools;

const COLOR_COUNT: usize = 256;

#[derive(Debug)]
pub struct Skybox {
    pub texture: Vec<Vec<Color>>,
}

impl Asset for Skybox {
    fn kind() -> Kind {
        Kind::Skybox
    }

    fn parse(input: &[u8], extension: Extension) -> crate::utils::nom::Result<Self> {
        match extension {
            Extension::Dat => {
                let (input, width) = number::le_u32(input)?;

                let (input, height) = number::le_u32(input)?;

                let (input, palette) = multi::count!(number::le_u16, 256)(input)?;
                let palette: Vec<_> = palette.into_iter().map(Color::from_12_bit).collect();

                let (input, texture) =
                    multi::count!(number::le_u8, (width * height) as usize)(input)?;
                let texture = texture
                    .into_iter()
                    .map(|c| palette[c as usize])
                    .chunks(width as usize)
                    .into_iter()
                    .map(Iterator::collect)
                    .collect();

                Ok((&[], Self { texture }))
            }
            _ => Err(error::ParseError::unsupported_extension(input, extension).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    #[ignore = "uses files that are local"]
    fn parse_works() -> eyre::Result<()> {
        let bytes = fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../output/deflated/19C57C.dat"
        ))?;

        let (_, skybox) = Skybox::parse(&bytes, Extension::Dat)?;

        let bytes: Vec<u8> = format!(
            "P6 {} {} 255\n",
            skybox.texture[0].len(),
            skybox.texture.len()
        )
        .bytes()
        .chain(
            skybox
                .texture
                .iter()
                .flat_map(|r| r.iter().flat_map(|c| [c.r, c.g, c.b]).collect::<Vec<_>>()),
        )
        .collect();

        // TODO(nenikitov): Make this uniform with other test (create directory if doesn't exist, etc)
        fs::write(
            concat!(env!("CARGO_MANIFEST_DIR"), "/../output/skyboxes/19C57C.ppm"),
            bytes,
        )?;

        Ok(())
    }
}
