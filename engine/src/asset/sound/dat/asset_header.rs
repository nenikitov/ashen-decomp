use crate::{
    asset::{extension::*, pack_info::PackInfo, AssetParser},
    utils::nom::*,
};

pub struct SoundAssetHeader {
    pub songs: PackInfo,
    pub effects: PackInfo,
    pub emitters: PackInfo,
    pub maps: PackInfo,
}

impl SoundAssetHeader {
    const HEADER: &'static str = "TSND";
}

impl AssetParser<Wildcard> for SoundAssetHeader {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
        move |input| {
            let (input, _) = bytes::tag(Self::HEADER)(input)?;

            let (input, songs) = PackInfo::parser(())(input)?;
            let (input, effects) = PackInfo::parser(())(input)?;
            let (input, emitters) = PackInfo::parser(())(input)?;
            let (input, maps) = PackInfo::parser(())(input)?;

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
}
