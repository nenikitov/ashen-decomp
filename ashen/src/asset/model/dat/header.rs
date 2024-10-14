use crate::{asset::AssetParser, utils::nom::*};

pub struct ModelHeader {
    pub triangle_count: u32,
    pub vertex_count: u32,
    pub texture_width: u32,
    pub texture_height: u32,
    pub frame_count: u32,
    pub frame_size: u32,
    pub sequence_count: u32,
    pub offset_texture: u32,
    pub offset_triangles: u32,
    pub offset_frames: u32,
    pub offset_sequences: u32,
    pub locator_nodes: [u8; 16],
}

impl AssetParser for ModelHeader {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, triangle_count) = number::le_u32(input)?;
            let (input, vertex_count) = number::le_u32(input)?;
            let (input, texture_width) = number::le_u32(input)?;
            let (input, texture_height) = number::le_u32(input)?;
            let (input, frame_count) = number::le_u32(input)?;
            let (input, frame_size) = number::le_u32(input)?;
            let (input, sequence_count) = number::le_u32(input)?;
            let (input, offset_texture) = number::le_u32(input)?;
            let (input, offset_triangles) = number::le_u32(input)?;
            let (input, offset_frames) = number::le_u32(input)?;
            let (input, offset_sequences) = number::le_u32(input)?;
            let (input, locator_nodes) = multi::count!(number::le_u8)(input)?;

            Ok((
                input,
                Self {
                    triangle_count,
                    vertex_count,
                    texture_width,
                    texture_height,
                    frame_count,
                    frame_size,
                    sequence_count,
                    offset_texture,
                    offset_triangles,
                    offset_frames,
                    offset_sequences,
                    locator_nodes,
                },
            ))
        }
    }
}
