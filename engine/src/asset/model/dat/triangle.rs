use crate::{asset::AssetChunkWithContext, utils::nom::*};

pub struct ModelPoint {
    pub vertex_index: u16,
    pub u: f32,
    pub v: f32,
}

pub struct TextureDimensions {
    pub width: u32,
    pub height: u32,
}

impl AssetChunkWithContext for ModelPoint {
    type Context<'a> = &'a TextureDimensions;

    fn parse(texture_dimensions: Self::Context<'_>) -> impl Fn(&[u8]) -> Result<Self> {
        move |input| {
            let (input, vertex_index) = number::le_u16(input)?;

            let (input, u) = number::le_u16(input)?;
            let u = (u as f32 + 0.5) / texture_dimensions.width as f32;

            let (input, v) = number::le_u16(input)?;
            // Y coordinates need to be flipped
            let v = 1f32 - (v as f32 + 0.5) / texture_dimensions.height as f32;

            Ok((input, Self { vertex_index, u, v }))
        }
    }
}

pub struct ModelTriangle {
    pub points: [ModelPoint; 3],
}

impl AssetChunkWithContext for ModelTriangle {
    type Context<'a> = TextureDimensions;

    fn parse(texture_dimensions: Self::Context<'_>) -> impl Fn(&[u8]) -> Result<Self> {
        move |input| {
            let (input, points) = multi::count!(ModelPoint::parse(&texture_dimensions))(input)?;

            Ok((input, Self { points }))
        }
    }
}
