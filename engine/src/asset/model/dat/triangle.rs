use crate::{
    asset::{extension::*, AssetParser},
    utils::nom::*,
};

pub struct ModelPoint {
    pub vertex_index: u16,
    pub u: f32,
    pub v: f32,
}

pub struct TextureDimensions {
    pub width: u32,
    pub height: u32,
}

impl AssetParser<Wildcard> for ModelPoint {
    type Context<'ctx> = &'ctx TextureDimensions;

    fn parser(texture_dimensions: Self::Context<'_>) -> impl FnParser<Self::Output> {
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

impl AssetParser<Wildcard> for ModelTriangle {
    type Context<'ctx> = TextureDimensions;

    fn parser(texture_dimensions: Self::Context<'_>) -> impl FnParser<Self::Output> {
        move |input| {
            let (input, points) = multi::count!(ModelPoint::parser(&texture_dimensions))(input)?;

            Ok((input, Self { points }))
        }
    }
}
