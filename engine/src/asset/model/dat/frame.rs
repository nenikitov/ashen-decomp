use crate::{asset::AssetChunk, utils::nom::*};
use fixed::types::I8F24;

// TODO(nenikitov): Should probably be a fancy utility class
// With generics for data type and dimension
#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl AssetChunk for Vec3 {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, x) = number::le_i32(input)?;
        let (input, y) = number::le_i32(input)?;
        let (input, z) = number::le_i32(input)?;

        Ok((
            input,
            Self {
                x: I8F24::from_bits(x).to_num(),
                y: I8F24::from_bits(y).to_num(),
                z: I8F24::from_bits(z).to_num(),
            },
        ))
    }
}

pub struct ModelVertex {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    // TODO(nenikitov): For now, no clue what this is for
    pub light_normal_index: u8,
}

impl AssetChunk for ModelVertex {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, x) = number::le_u8(input)?;
        let (input, y) = number::le_u8(input)?;
        let (input, z) = number::le_u8(input)?;
        let (input, light_normal_index) = number::le_u8(input)?;

        Ok((
            input,
            Self {
                x,
                y,
                z,
                light_normal_index,
            },
        ))
    }
}

impl ModelVertex {
    fn to_parsed(&self, scale: &Vec3, scale_origin: &Vec3) -> ModelVertexParsed {
        macro_rules! transform {
            ($coordinate: ident, $origin: expr) => {
                scale_origin.$coordinate
                    - scale.$coordinate
                        * ((self.$coordinate as f32 - $origin) + scale_origin.$coordinate)
            };
        }
        ModelVertexParsed {
            x: transform!(x, 128.0),
            y: transform!(y, 128.0),
            z: transform!(z, 256.0),
            normal_index: self.light_normal_index,
        }
    }
}

pub struct ModelVertexParsed {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub normal_index: u8,
}

pub struct ModelFrame {
    // TODO(nenikitov): In game those are fixed point values and not integers
    pub bounding_sphere_radius: i32,
    pub vertices: Vec<ModelVertexParsed>,
    pub triangle_normal_indexes: Vec<u8>,
}

impl ModelFrame {
    pub fn parse(
        vertex_count: usize,
        triangle_count: usize,
        frame_size: usize,
    ) -> impl Fn(&[u8]) -> Result<Self> {
        move |input| {
            let (input, scale) = Vec3::parse(input)?;
            let (input, scale_origin) = Vec3::parse(input)?;

            let (input, bounding_sphere_radius) = number::le_i32(input)?;

            let (input, vertices) = multi::count!(ModelVertex::parse, vertex_count)(input)?;
            let vertices = vertices
                .into_iter()
                .map(|v| v.to_parsed(&scale, &scale_origin))
                .collect();

            let (input, triangle_normal_indexes) =
                multi::count!(number::le_u8, triangle_count)(input)?;

            // This ugly formula calculates the padding after the frame data until next frame data
            // ```
            // frame_size
            //    - sizeof(scale)
            //    - sizeof(scale_origin)
            //    - sizeof(bounding_sphere_radius)
            //    - sizeof(vertices)                // sizeof(ModelVertex) * vertex_count
            //    - sizeof(triangle_normalindexes)  // sizeof(u8) triangle_count
            // ```
            let (input, _) =
                bytes::take(frame_size - 28 - 4 * vertex_count - triangle_count)(input)?;

            Ok((
                input,
                Self {
                    bounding_sphere_radius,
                    vertices,
                    triangle_normal_indexes,
                },
            ))
        }
    }
}
