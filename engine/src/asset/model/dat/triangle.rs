use crate::{asset::AssetChunk, utils::nom::*};

pub struct ModelPoint {
    pub vertex_index: u16,
    pub u: u16,
    pub v: u16,
}

impl AssetChunk for ModelPoint {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, vertex_index) = number::le_u16(input)?;
        let (input, u) = number::le_u16(input)?;
        let (input, v) = number::le_u16(input)?;

        Ok((input, Self { vertex_index, u, v }))
    }
}

pub struct ModelTriangle {
    pub points: [ModelPoint; 3],
}

impl AssetChunk for ModelTriangle {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, points) = multi::count!(ModelPoint::parse)(input)?;

        Ok((input, Self { points }))
    }
}
