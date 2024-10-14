use crate::{
    asset::{pack_info::PackInfo, AssetParser},
    utils::nom::*,
};

pub struct SoundChunkHeader {
    pub infos: Vec<PackInfo>,
}

impl AssetParser for SoundChunkHeader {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, count) = number::le_u32(input)?;
            let (input, infos) = multi::count!(PackInfo::parser(()), count as usize)(input)?;

            Ok((input, Self { infos }))
        }
    }
}
