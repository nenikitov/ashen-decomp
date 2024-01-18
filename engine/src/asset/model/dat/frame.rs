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

pub(crate) struct VertexTransform<'a> {
    scale: &'a Vec3,
    origin: &'a Vec3,
}

impl AssetChunkWithContext for ModelVertex {
    type Context<'a> = &'a VertexTransform<'a>;

    fn parse(transform: Self::Context<'_>) -> impl Fn(&[u8]) -> Result<Self> {
        macro_rules! transform {
            ($coordinate: ident) => {
                (transform.scale.$coordinate * $coordinate as f32 / -256.0
                    - transform.origin.$coordinate)
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

pub struct ModelSpecs {
    pub vertex_count: u32,
    pub triangle_count: u32,
    pub frame_size: u32,
}

impl AssetChunkWithContext for ModelFrame {
    type Context<'a> = &'a ModelSpecs;

    fn parse(model_specs: Self::Context<'_>) -> impl Fn(&[u8]) -> Result<Self> {
        move |input| {
            let (input, scale) = Vec3::parse(input)?;
            let (input, origin) = Vec3::parse(input)?;

            let (input, bounding_sphere_radius) = number::le_i24f8(input)?;

            let (input, vertices) = multi::count!(
                ModelVertex::parse(&VertexTransform {
                    scale: &scale,
                    origin: &origin
                }),
                model_specs.vertex_count as usize
            )(input)?;

            let (input, triangle_normal_indexes) =
                multi::count!(number::le_u8, model_specs.triangle_count as usize)(input)?;

            // This ugly formula calculates the padding after the frame data until next frame data
            // ```
            // frame_size
            //    - sizeof(scale)
            //    - sizeof(scale_origin)
            //    - sizeof(bounding_sphere_radius)
            //    - sizeof(vertices)                // sizeof(ModelVertex) * vertex_count
            //    - sizeof(triangle_normalindexes)  // sizeof(u8) triangle_count
            // ```
            let (input, _) = bytes::take(
                model_specs.frame_size
                    - 28
                    - 4 * model_specs.vertex_count
                    - model_specs.triangle_count,
            )(input)?;

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
