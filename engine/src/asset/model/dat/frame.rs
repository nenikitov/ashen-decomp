use crate::{
    asset::{AssetChunk, AssetChunkWithContext},
    utils::nom::*,
};

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
        let (input, x) = number::le_i16f16(input)?;
        let (input, y) = number::le_i16f16(input)?;
        let (input, z) = number::le_i16f16(input)?;

        Ok((
            input,
            Self {
                x: x.to_num(),
                y: y.to_num(),
                z: z.to_num(),
            },
        ))
    }
}

pub struct ModelVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    // TODO(nenikitov): For now, no clue what this is for
    pub normal_index: u8,
}

impl ModelVertex {
    const UNITS_PER_METER: f32 = 32.0;
}

impl AssetChunkWithContext for ModelVertex {
    type Context<'a> = (&'a Vec3, &'a Vec3);

    fn parse((scale, scale_origin): Self::Context<'_>) -> impl Fn(&[u8]) -> Result<Self> {
        macro_rules! transform {
            ($coordinate: ident) => {
                (scale.$coordinate * $coordinate as f32 / -256.0 - scale_origin.$coordinate)
                    / Self::UNITS_PER_METER
            };
        }

        move |input| {
            let (input, x) = number::le_u8(input)?;
            let (input, y) = number::le_u8(input)?;
            let (input, z) = number::le_u8(input)?;
            let (input, normal_index) = number::le_u8(input)?;

            Ok((
                input,
                Self {
                    x: transform!(x),
                    y: transform!(y),
                    z: transform!(z),
                    normal_index,
                },
            ))
        }
    }
}

pub struct ModelFrame {
    pub bounding_sphere_radius: f32,
    pub vertices: Vec<ModelVertex>,
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

            let (input, bounding_sphere_radius) = number::le_i24f8(input)?;

            let (input, vertices) =
                multi::count!(ModelVertex::parse((&scale, &scale_origin)), vertex_count)(input)?;

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
                    bounding_sphere_radius: bounding_sphere_radius.to_num(),
                    vertices,
                    triangle_normal_indexes,
                },
            ))
        }
    }
}
