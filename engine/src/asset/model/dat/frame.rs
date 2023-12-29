use crate::{asset::AssetChunk, utils::nom::*};
use fixed::types::I8F24;

// TODO(nenikitov): Should probably be a fancy utility class
// With generics for data type and dimension
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
        ModelVertexParsed {
            x: scale_origin.x - scale.x * (self.x as f32 - scale_origin.x),
            y: scale_origin.y - scale.y * (self.y as f32 - scale_origin.y),
            z: scale_origin.z - scale.z * (self.z as f32 - scale_origin.z),
            light_normal_index: self.light_normal_index,
        }
    }
}

pub struct ModelVertexParsed {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub light_normal_index: u8,
}

pub struct ModelFrame {
    // TODO(nenikitov): In game those are fixed point values and not integers
    pub bounding_sphere_radius: i32,
    pub vertices: Vec<ModelVertexParsed>,
}

impl ModelFrame {
    pub fn parse(vertex_count: usize, frame_size: usize) -> impl Fn(&[u8]) -> Result<Self> {
        move |input| {
            let (input, scale) = Vec3::parse(input)?;
            let (input, scale_origin) = Vec3::parse(input)?;

            let (input, bounding_sphere_radius) = number::le_i32(input)?;

            let (input, vertices) = multi::count!(ModelVertex::parse, vertex_count)(input)?;
            let vertices = vertices
                .into_iter()
                .map(|v| v.to_parsed(&scale, &scale_origin))
                .collect();

            // TODO(nenikitov): Figure out what this data does
            // This ugly formula is probably not needed when I figure out what the data does
            // ```
            // frame_size
            //    - sizeof(scale)
            //    - sizeof(scale_origin)
            //    - sizeof(bounding_sphere_radius)
            //    - sizeof(vertices) // sizeof(ModelVertex) * vertex_count
            // ```
            let (input, _) = bytes::take(frame_size - 28 - 4 * vertex_count)(input)?;

            Ok((
                input,
                Self {
                    bounding_sphere_radius,
                    vertices,
                },
            ))
        }
    }
}
