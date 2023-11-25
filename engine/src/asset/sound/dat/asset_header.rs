use crate::{
    asset::{pack_info::PackInfo, AssetChunk},
    utils::nom::*,
};

pub struct SongAssetHeader {
    pub songs: PackInfo,
    pub effects: PackInfo,
    pub emitters: PackInfo,
    pub maps: PackInfo,
}

impl SongAssetHeader {
    const HEADER: &'static str = "TSND";
}

impl AssetChunk for SongAssetHeader {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, _) = bytes::tag(Self::HEADER)(input)?;

        let (input, songs) = PackInfo::parse(input)?;
        let (input, effects) = PackInfo::parse(input)?;
        let (input, emitters) = PackInfo::parse(input)?;
        let (input, maps) = PackInfo::parse(input)?;

        Ok((
            input,
            Self {
                songs,
                effects,
                emitters,
                maps,
            },
        ))
    }
}
