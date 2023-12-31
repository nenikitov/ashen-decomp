use crate::utils::nom::*;

pub struct ModelPoint {
    pub vertex_index: u16,
    pub u: f32,
    pub v: f32,
}

impl ModelPoint {
    pub fn parse(texture_width: u32, texture_height: u32) -> impl Fn(&[u8]) -> Result<Self> {
        move |input| {
            let (input, vertex_index) = number::le_u16(input)?;

            let (input, u) = number::le_u16(input)?;
            let u = (u as f32 + 0.5) / texture_width as f32;

            let (input, v) = number::le_u16(input)?;
            // Y coordinates need to be flipped
            let v = 1f32 - (v as f32 + 0.5) / texture_height as f32;

            Ok((input, Self { vertex_index, u, v }))
        }
    }
}

pub struct ModelTriangle {
    pub points: [ModelPoint; 3],
}

impl ModelTriangle {
    pub fn parse(texture_width: u32, texture_height: u32) -> impl Fn(&[u8]) -> Result<Self> {
        move |input| {
            let (input, points) =
                multi::count!(ModelPoint::parse(texture_width, texture_height))(input)?;

            Ok((input, Self { points }))
        }
    }
}
