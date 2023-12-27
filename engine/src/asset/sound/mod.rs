mod dat;

use flate2::read::ZlibDecoder;
use std::{
    fs,
    io::{self, Read},
    ops::Index,
    path::Path,
};

use self::dat::{mixer::SoundEffect, t_effect::TEffect};

use super::{pack_info::PackInfo, Asset, AssetChunk, Extension, Kind};
use crate::{
    asset::sound::dat::{
        asset_header::SoundAssetHeader, chunk_header::SoundChunkHeader, t_song::TSong,
    },
    error::{self, ParseError},
    utils::{format::*, nom::*},
};

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
    const CHANNEL_COUNT: usize = 2;

    fn save_songs<P>(&self, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        self.songs.iter().enumerate().try_for_each(|(i, song)| {
            crate::utils::fs::output_file(
                format!("{i:X}.wav"),
                song.mix().to_wave(Self::SAMPLE_RATE, Self::CHANNEL_COUNT),
            )
        })
    }

    fn save_effects<P>(&self, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        self.effects.iter().enumerate().try_for_each(|(i, effect)| {
            crate::utils::fs::output_file(
                format!("{i:X}.wav"),
                effect.mix().to_wave(Self::SAMPLE_RATE, Self::CHANNEL_COUNT),
            )
        })
    }
}

impl Asset for SoundAssetCollection {
    fn kind() -> Kind {
        Kind::SoundCollection
    }

    fn parse(input: &[u8], extension: Extension) -> Result<Self> {
        match extension {
            Extension::Dat => {
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
            _ => Err(error::ParseError::unsupported_extension(input, extension).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fs::*;
    use std::cell::LazyCell;

    const SOUND_DATA: LazyCell<Vec<u8>> = LazyCell::new(|| {
        fs::read(workspace_file!("output/deflated/BBC974.dat")).expect("deflated test ran")
    });

    #[test]
    #[ignore = "uses files that are local"]
    fn output_songs() -> eyre::Result<()> {
        let (_, sac) = SoundAssetCollection::parse(&SOUND_DATA, Extension::Dat)?;
        sac.save_songs(workspace_file!("output/sounds/songs/"))?;

        Ok(())
    }

    #[test]
    #[ignore = "uses files that are local"]
    fn output_effects() -> eyre::Result<()> {
        let (_, sac) = SoundAssetCollection::parse(&SOUND_DATA, Extension::Dat)?;
        sac.save_effects(workspace_file!("output/sounds/effects/"))?;

        Ok(())
    }
}
