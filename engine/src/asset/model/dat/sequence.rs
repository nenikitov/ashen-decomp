use crate::{asset::AssetChunkWithContext, utils::nom::*};

pub struct ModelSequence {
    pub frames: Vec<u32>,
}

impl AssetChunkWithContext for ModelSequence {
    type Context<'a> = &'a [u8];

    fn parse(full_input: Self::Context<'_>) -> impl Fn(&[u8]) -> Result<Self> {
        move |input| {
            let (input, frame_count) = number::le_u32(input)?;
            let (input, offset) = number::le_u32(input)?;

            let (_, frames) = multi::count!(number::le_u32, frame_count as usize)(
                &full_input[offset as usize..],
            )?;

            Ok((input, Self { frames }))
        }
    }
}
