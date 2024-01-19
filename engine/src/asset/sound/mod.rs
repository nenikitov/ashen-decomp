mod dat;

use self::dat::t_effect::TEffect;

use super::{Asset, AssetChunk, Extension, Kind};
use crate::{
    asset::sound::dat::{
        asset_header::SoundAssetHeader, chunk_header::SoundChunkHeader, t_song::TSong,
    },
    error,
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

impl Asset for SoundAssetCollection {
    type Context = ();

    fn kind() -> Kind {
        Kind::SoundCollection
    }

    fn parse(input: &[u8], extension: Extension, _: Self::Context) -> Result<Self> {
        match extension {
            Extension::Dat => {
                let (_, header) = SoundAssetHeader::parse(input)?;

                let (_, songs) = SoundChunkHeader::parse(&input[header.songs])?;
                let songs = songs
                    .infos
                    .into_iter()
                    .map(|s| decompress(&input[s]))
                    .map(|s| TSong::parse(s.as_slice()).map(|(_, d)| d))
                    .collect::<std::result::Result<Vec<_>, _>>()?;

                let (_, effects) = SoundChunkHeader::parse(&input[header.effects])?;
                let effects = effects
                    .infos
                    .into_iter()
                    .map(|s| decompress(&input[s]))
                    .map(|s| TEffect::parse(s.as_slice()).map(|(_, d)| d))
                    .collect::<std::result::Result<Vec<_>, _>>()?;

                Ok((&[], SoundAssetCollection { songs, effects }))
            }
            _ => Err(error::ParseError::unsupported_extension(input, extension).into()),
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
        let (_, asset) = SoundAssetCollection::parse(&SOUND_DATA, Extension::Dat, ())?;

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
