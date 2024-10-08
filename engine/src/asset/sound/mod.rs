mod dat;
pub(crate) mod sample;

use self::{dat::mixer::TSongMixer, sample::AudioBuffer};
use super::{extension::*, AssetParser};
use crate::{
    asset::sound::dat::{
        asset_header::SoundAssetHeader, chunk_header::SoundChunkHeader, t_effect::TEffect,
        t_song::TSong,
    },
    utils::{compression::decompress, nom::*},
};

pub enum Sound {
    Song(TSong),
    Effect(TEffect),
}

impl Sound {
    pub fn mix(&self) -> AudioBuffer<i16> {
        match self {
            Sound::Song(sound) => sound.mix(),
            Sound::Effect(effect) => effect.mix(),
        }
    }
}

pub struct SoundCollection;

impl SoundCollection {
    pub const SAMPLE_RATE: usize = 16000;
    pub const CHANNEL_COUNT: usize = 1;
}

impl AssetParser<Pack> for SoundCollection {
    type Output = Vec<Sound>;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (_, header) = SoundAssetHeader::parser(())(input)?;

            let (_, songs) = SoundChunkHeader::parser(())(&input[header.songs])?;
            let songs = songs
                .infos
                .into_iter()
                .map(|s| decompress(&input[s]))
                .map(|s| TSong::parser(())(s.as_slice()).map(|(_, d)| d))
                .map(|s| s.map(Sound::Song));

            let (_, effects) = SoundChunkHeader::parser(())(&input[header.effects])?;
            let effects = effects
                .infos
                .into_iter()
                .map(|s| decompress(&input[s]))
                .map(|s| TEffect::parser(())(s.as_slice()).map(|(_, d)| d))
                .map(|s| s.map(Sound::Effect));

            let sounds = songs
                .chain(effects)
                .collect::<std::result::Result<Vec<_>, _>>()?;

            Ok((&[], sounds))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, path::PathBuf};

    use super::*;
    use crate::utils::{format::*, test::*};

    const SOUND_DATA: LazyCell<Vec<u8>> = deflated_file!("97.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, sounds) = <SoundCollection as AssetParser<Pack>>::parser(())(&SOUND_DATA)?;

        let output_dir = PathBuf::from(parsed_file_path!("sounds/songs/"));

        // TODO(nenikitov): Remove this debug code
        {
            let i = 0x3;
            let song = sounds
                .iter()
                .filter_map(|s| match s {
                    Sound::Song(s) => Some(s),
                    Sound::Effect(_) => None,
                })
                .collect::<Vec<_>>()[i];

            //dbg!(song);
        }

        sounds
            .iter()
            .filter_map(|s| {
                if let Sound::Song(song) = s {
                    Some((s, song))
                } else {
                    None
                }
            })
            .enumerate()
            .try_for_each(|(i, (song, t))| -> std::io::Result<()> {
                let file = output_dir.join(format!("{i:0>2X}.wav"));
                println!("# SONG {i}");
                output_file(file, song.mix().to_wave())?;

                let file = output_dir.join(format!("{i:0>2X}.txt"));
                output_file(file, format!("{t:#?}"))?;

                Ok(())
            })?;

        let output_dir = PathBuf::from(parsed_file_path!("sounds/effects/"));

        sounds
            .iter()
            .filter(|s| matches!(s, Sound::Effect(_)))
            .enumerate()
            .try_for_each(|(i, effect)| {
                let file = output_dir.join(format!("{i:0>2X}.wav"));
                output_file(file, effect.mix().to_wave())
            })?;

        Ok(())
    }
}
