use crate::{
    asset::{pack_info::PackInfo, AssetChunk},
    utils::nom::*,
};

pub struct SongChunkHeader {
    pub songs: Vec<PackInfo>,
}

impl AssetChunk for SongChunkHeader {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, count) = number::le_u32(input)?;
        let (input, songs) = multi::count!(PackInfo::parse, count as usize)(input)?;

        Ok((input, Self { songs }))
    }
}
