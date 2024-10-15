use crate::{asset::Parser, utils::nom::*};

pub struct ModelSequence {
    pub frames: Vec<u32>,
}

impl Parser for ModelSequence {
    // TODO(nenikitov): Maybe refactor it to not accept full input.
    // In other asset parts, it's parent's responsability to cut input into slices
    // for asset parts to parse.
    type Context<'ctx> = Input<'ctx>;

    fn parser(full_input: Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
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
