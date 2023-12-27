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
    use super::*;
    use crate::utils::{format::*, fs::*};
    use std::{cell::LazyCell, fs};

    const SKYBOX_DATA: LazyCell<Vec<u8>> = LazyCell::new(|| {
        fs::read(workspace_file!("output/deflated/19C57C.dat")).expect("deflated test ran")
    });

    #[test]
    #[ignore = "uses files that are local"]
    fn parse_works() -> eyre::Result<()> {
        let (_, skybox) = Skybox::parse(&SKYBOX_DATA, Extension::Dat)?;

        output_file(
            workspace_file!("output/skyboxes/19C57C.ppm"),
            skybox.texture.to_ppm(),
        )?;

        Ok(())
    }
}
