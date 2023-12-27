mod dat;

use itertools::Itertools;

use super::{Asset, Extension, Kind};
use crate::{
    asset::{model::dat::header::ModelHeader, AssetChunk},
    error,
    utils::nom::*,
};

pub struct Model {
    texture: Vec<Vec<u8>>,
}

impl Asset for Model {
    fn kind() -> super::Kind {
        Kind::Model
    }

    fn parse(input: &[u8], extension: Extension) -> Result<Self> {
        match extension {
            Extension::Dat => {
                let (_, header) = ModelHeader::parse(input)?;

                let (_, texture) = multi::count!(
                    number::le_u8,
                    (header.texture_width * header.texture_height) as usize
                )(&input[header.offset_texture as usize..])?;
                let texture = texture
                    .into_iter()
                    .chunks(header.texture_width as usize)
                    .into_iter()
                    .map(Iterator::collect)
                    .collect();

                Ok((&[0], Self { texture }))
            }
            _ => Err(error::ParseError::unsupported_extension(input, extension).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset::color_map::{ColorMap, PaletteTexture},
        utils::{format::*, test::*},
    };
    use std::cell::LazyCell;

    const PICKUP_COLOR_MAP_DATA: LazyCell<Vec<u8>> = deflated_file!("04.dat");
    const MODEL_DATA: LazyCell<Vec<u8>> = deflated_file!("2D.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, model) = Model::parse(&MODEL_DATA, Extension::Dat)?;
        let (_, color_map) = ColorMap::parse(&PICKUP_COLOR_MAP_DATA, Extension::Dat)?;

        output_file(
            parsed_file_path!("models/pickup-pistol.ppm"),
            model.texture.with_palette(&color_map.shades[15]).to_ppm(),
        )?;

        Ok(())
    }
}
