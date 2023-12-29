mod dat;

use itertools::Itertools;

use self::dat::triangle::ModelTriangle;

use super::{Asset, Extension, Kind};
use crate::{
    asset::{
        model::dat::{
            frame::ModelFrame,
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
    pub frames: Vec<ModelFrame>,
}

impl Asset for Model {
    fn kind() -> super::Kind {
        Kind::Model
    }

    fn parse(input: &[u8], extension: Extension) -> Result<Self> {
        match extension {
            Extension::Dat => {
                let (_, header) = ModelHeader::parse(input)?;

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

                let (_, frames) = multi::count!(
                    ModelFrame::parse(header.vertex_count as usize, header.frame_size as usize),
                    header.frame_count as usize
                )(&input[header.offset_frames as usize..])?;

                Ok((
                    &[0],
                    Self {
                        triangles,
                        texture,
                        sequences,
                        frames,
                    },
                ))
            }
            _ => Err(error::ParseError::unsupported_extension(input, extension).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{dat::frame::ModelVertexParsed, *};
    use crate::{
        asset::color_map::{ColorMap, PaletteTexture},
        utils::{format::*, test::*},
    };
    use std::{cell::LazyCell, fmt::Display, path::PathBuf};

    const COLOR_MAP_DATA: LazyCell<Vec<u8>> = deflated_file!("04.dat");
    const MODEL_DATA: LazyCell<Vec<u8>> = deflated_file!("0E.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, model) = Model::parse(&MODEL_DATA, Extension::Dat)?;
        let (_, color_map) = ColorMap::parse(&COLOR_MAP_DATA, Extension::Dat)?;

        let output_dir = PathBuf::from(parsed_file_path!("models/hunter/"));

        output_file(
            output_dir.join("hunter.ppm"),
            model.texture.with_palette(&color_map.shades[15]).to_ppm(),
        )?;

        output_file(output_dir.join("hunter.py"), model.to_py())?;

        dbg!(model.sequences);

        Ok(())
    }

    pub trait ModelPythonFile {
        fn to_py(&self) -> String;
    }

    impl ModelPythonFile for Model {
        fn to_py(&self) -> String {
            format!(
                r#"\
import bpy

vertices = [
{}
]
triangles = [
{}
]

for obj in bpy.data.objects:
    bpy.data.objects.remove(obj, do_unlink=True)

mesh = bpy.data.meshes.new("Mesh")
object = bpy.data.objects.new("Model", mesh)

mesh.from_pydata(
    [
        (v["x"], v["y"], v["z"])
        for v in vertices
    ],
    [],
    [
        (t["points"][0]["vertex_index"], t["points"][1]["vertex_index"], t["points"][2]["vertex_index"])
        for t in triangles
    ]
)

bpy.context.collection.objects.link(object)
            "#,
                self.frames[97]
                    .vertices
                    .iter()
                    .map(|v| format!("    {v}"))
                    .join(",\n"),
                self.triangles
                    .iter()
                    .map(|v| format!("    {v}"))
                    .join(",\n"),
            )
        }
    }

    impl Display for ModelVertexParsed {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                r#"{{ "x": {}, "y": {}, "z": {} }}"#,
                self.x, self.y, self.z
            )
        }
    }

    impl Display for ModelTriangle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                r#"{{ "points": [{{ "vertex_index": {} }}, {{ "vertex_index": {} }}, {{ "vertex_index": {} }}] }}"#,
                self.points[0].vertex_index,
                self.points[1].vertex_index,
                self.points[2].vertex_index,
            )
        }
    }
}
