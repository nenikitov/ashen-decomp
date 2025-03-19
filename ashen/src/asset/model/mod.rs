mod dat;

use dat::{
    frame::{ModelFrame, ModelSpecs},
    header::ModelHeader,
    sequence::ModelSequence,
    triangle::{ModelTriangle, TextureDimensions},
};

use super::{
    Parser,
    color_map::Color,
    texture::{Texture, TextureSize},
};
use crate::utils::{format::ModelPythonFile, nom::*};

pub struct Model {
    pub texture: Texture,
    pub triangles: Vec<ModelTriangle>,
    pub sequences: Vec<ModelSequence>,
    pub frames: Vec<ModelFrame>,
}

impl Parser for Model {
    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
        move |input| {
            let (_, header) = ModelHeader::parser(())(input)?;

            let (_, triangles) = multi::count!(
                ModelTriangle::parser(TextureDimensions {
                    width: header.texture_width,
                    height: header.texture_height
                }),
                header.triangle_count as usize
            )(&input[header.offset_triangles as usize..])?;

            let (_, texture) = Texture::parser(TextureSize {
                width: header.texture_width as usize,
                height: header.texture_height as usize,
            })(&input[header.offset_texture as usize..])?;

            let (_, sequences) = multi::count!(
                ModelSequence::parser(input),
                header.sequence_count as usize
            )(&input[header.offset_sequences as usize..])?;

            let (_, frames) = multi::count!(
                ModelFrame::parser(ModelSpecs {
                    vertex_count: header.vertex_count,
                    triangle_count: header.triangle_count,
                    frame_size: header.frame_size
                }),
                header.frame_count as usize
            )(&input[header.offset_frames as usize..])?;

            Ok((
                &[],
                Self {
                    texture,
                    triangles,
                    sequences,
                    frames,
                },
            ))
        }
    }
}

impl Model {
    // TODO(Unavailable): Could provide conversions to gif using `shadybug`.

    pub fn to_blender_script<P>(&self, path: P, palette: &[Color]) -> std::io::Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        let bytes = self.to_py(palette);
        std::fs::write(path, bytes)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, path::PathBuf};

    use super::Model;
    use crate::{
        asset::{Parser, color_map::ColorMap},
        utils::{format::ModelPythonFile, test::*},
    };

    const COLOR_MAP_DATA: LazyCell<Vec<u8>> = deflated_file!("01.dat");
    const MODEL_DATA: LazyCell<Vec<u8>> = deflated_file!("0E-deflated.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, model) = Model::parser(())(&MODEL_DATA)?;
        let palette = {
            let (_, color_map) = ColorMap::parser(())(&COLOR_MAP_DATA)?;
            color_map.shades[15]
        };

        let output_dir = PathBuf::from(parsed_file_path!("models/hunter/"));

        output_file(output_dir.join("hunter.py"), model.to_py(&palette))?;

        Ok(())
    }
}
