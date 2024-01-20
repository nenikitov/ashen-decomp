mod dat;

use super::{extension::Pack, AssetChunk, AssetParser};
use crate::{
    asset::sound::dat::{
        asset_header::SoundAssetHeader, chunk_header::SoundChunkHeader, t_effect::TEffect,
        t_song::TSong,
    },
    utils::nom::*,
};
use flate2::read::ZlibDecoder;
use std::io::Read;

// TODO(nenikitov): Move to utils
// TODO(nenikitov): Return `Result`
fn deflate(input: &[u8]) -> Vec<u8> {
    if let [b'Z', b'L', s1, s2, s3, bytes @ ..] = input {
        let size = u32::from_le_bytes([*s1, *s2, *s3, 0]);

        let mut decoder = ZlibDecoder::new(bytes);
        let mut data = Vec::with_capacity(size as usize);
        decoder
            .read_to_end(&mut data)
            .expect("Data should be a valid zlib stream");

        data
    } else {
        input.to_vec()
    }
}

pub struct SoundAssetCollection {
    songs: Vec<TSong>,
    effects: Vec<TEffect>,
}

impl SoundAssetCollection {
    const SAMPLE_RATE: usize = 16000;
    const CHANNEL_COUNT: usize = 1;
}

impl AssetParser<Pack> for SoundAssetCollection {
    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
        move |input| {
            let (_, header) = SoundAssetHeader::parse(input)?;

            let (_, songs) = SoundChunkHeader::parse(&input[header.songs])?;
            let songs = songs
                .infos
                .into_iter()
                .map(|s| deflate(&input[s]))
                .map(|s| TSong::parse(s.as_slice()).map(|(_, d)| d))
                .collect::<std::result::Result<Vec<_>, _>>()?;

            let (_, effects) = SoundChunkHeader::parse(&input[header.effects])?;
            let effects = effects
                .infos
                .into_iter()
                .map(|s| deflate(&input[s]))
                .map(|s| TEffect::parse(s.as_slice()).map(|(_, d)| d))
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
