use crate::{asset::AssetChunk, utils::nom::*};

pub struct ModelSequence {
    pub frame_count: u32,
    pub offset: u32,
}

impl AssetChunk for ModelSequence {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, frame_count) = number::le_u32(input)?;
        let (input, offset) = number::le_u32(input)?;

        Ok((
            input,
            Self {
                frame_count,
                offset,
            },
        ))
    }
}

#[derive(Debug)]
pub struct ModelSequenceParsed {
    pub frames: Vec<u32>,
}

impl ModelSequenceParsed {
    pub fn parse<'a>(input: &'a [u8], header: &ModelSequence) -> Result<'a, Self> {
        let (_, frames) = multi::count!(number::le_u32, header.frame_count as usize)(
            &input[header.offset as usize..],
        )?;

        Ok((&[], Self { frames }))
    }
}
