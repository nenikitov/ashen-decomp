mod dat;

use super::{extension::*, AssetParser};
use crate::{
    asset::sound::dat::{
        asset_header::SoundAssetHeader, chunk_header::SoundChunkHeader, t_effect::TEffect,
        t_song::TSong,
    },
    utils::{compression::decompress, nom::*},
};

pub struct SoundAssetCollection {
    songs: Vec<TSong>,
    effects: Vec<TEffect>,
}

impl SoundAssetCollection {
    const SAMPLE_RATE: usize = 16000;
    const CHANNEL_COUNT: usize = 1;
}

impl AssetParser<Pack> for SoundAssetCollection {
    // TODO(nenikitov): Make it output vecs somehow to follow collection convention
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
        move |input| {
            let (_, header) = SoundAssetHeader::parser(())(input)?;

            let (_, songs) = SoundChunkHeader::parser(())(&input[header.songs])?;
            let songs = songs
                .infos
                .into_iter()
                .map(|s| decompress(&input[s]))
                .map(|s| TSong::parser(())(s.as_slice()).map(|(_, d)| d))
                .collect::<std::result::Result<Vec<_>, _>>()?;

            let (_, effects) = SoundChunkHeader::parser(())(&input[header.effects])?;
            let effects = effects
                .infos
                .into_iter()
                .map(|s| decompress(&input[s]))
                .map(|s| TEffect::parser(())(s.as_slice()).map(|(_, d)| d))
                .collect::<std::result::Result<Vec<_>, _>>()?;

            Ok((&[], SoundAssetCollection { songs, effects }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{format::*, test::*};
    use std::{cell::LazyCell, path::PathBuf};

    const SOUND_DATA: LazyCell<Vec<u8>> = deflated_file!("97.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, asset) = <SoundAssetCollection as AssetParser<Pack>>::parser(())(&SOUND_DATA)?;

        let output_dir = PathBuf::from(parsed_file_path!("sounds/songs/"));

        asset.songs.iter().enumerate().try_for_each(|(i, song)| {
            let file = output_dir.join(format!("{i:0>2X}.wav"));
            output_file(
                file,
                song.mix().to_wave(
                    SoundAssetCollection::SAMPLE_RATE,
                    SoundAssetCollection::CHANNEL_COUNT,
                ),
            )
        })?;

        let output_dir = PathBuf::from(parsed_file_path!("sounds/effects/"));

        asset
            .effects
            .iter()
            .enumerate()
            .try_for_each(|(i, effect)| {
                let file = output_dir.join(format!("{i:0>2X}.wav"));
                output_file(
                    file,
                    effect.mix().to_wave(
                        SoundAssetCollection::SAMPLE_RATE,
                        SoundAssetCollection::CHANNEL_COUNT,
                    ),
                )
            })?;

        Ok(())
    }
}
