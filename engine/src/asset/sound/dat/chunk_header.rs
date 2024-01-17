use crate::{
    asset::{pack_info::PackInfo, AssetChunk},
    utils::nom::*,
};

pub struct SoundChunkHeader {
    pub infos: Vec<PackInfo>,
}

impl AssetChunk for SoundChunkHeader {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, count) = number::le_u32(input)?;
        let (input, infos) = multi::count!(PackInfo::parse, count as usize)(input)?;

        Ok((input, Self { infos }))
    }
}
