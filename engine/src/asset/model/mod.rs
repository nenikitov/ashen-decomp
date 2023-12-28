mod dat;

use itertools::Itertools;

use self::dat::triangle::ModelTriangle;

use super::{Asset, Extension, Kind};
use crate::{
    asset::{
        model::dat::{
            header::ModelHeader,
            sequence::{ModelSequence, ModelSequenceParsed},
        },
        AssetChunk,
    },
    error,
    utils::nom::*,
};

pub struct Model {
    pub texture: Vec<Vec<u8>>,
    pub triangles: Vec<ModelTriangle>,
    pub sequences: Vec<ModelSequenceParsed>,
}

impl Asset for Model {
    fn kind() -> super::Kind {
        Kind::Model
    }

    fn parse(input: &[u8], extension: Extension) -> Result<Self> {
        match extension {
            Extension::Dat => {
                let (_, header) = ModelHeader::parse(input)?;

                dbg!(header.vertex_count);

                let (_, triangles) = multi::count!(
                    ModelTriangle::parse,
                    header.triangle_count as usize
                )(&input[header.offset_triangles as usize..])?;

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

                let (_, sequences) = multi::count!(
                    ModelSequence::parse,
                    header.sequence_count as usize
                )(&input[header.offset_sequences as usize..])?;
                let sequences = sequences
                    .into_iter()
                    .map(|s| ModelSequenceParsed::parse(input, s).map(|(_, d)| d))
                    .collect::<std::result::Result<Vec<_>, _>>()?;

                Ok((
                    &[0],
                    Self {
                        triangles,
                        texture,
                        sequences,
                    },
                ))
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

    const COLOR_MAP_DATA: LazyCell<Vec<u8>> = deflated_file!("01.dat");
    const MODEL_DATA: LazyCell<Vec<u8>> = deflated_file!("0E.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, model) = Model::parse(&MODEL_DATA, Extension::Dat)?;
        let (_, color_map) = ColorMap::parse(&COLOR_MAP_DATA, Extension::Dat)?;

        dbg!(model.sequences);

        output_file(
            parsed_file_path!("models/hunter.ppm"),
            model.texture.with_palette(&color_map.shades[15]).to_ppm(),
        )?;

        Ok(())
    }
}
