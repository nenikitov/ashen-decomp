mod dat;
pub(crate) mod sample;

use self::{dat::mixer::TSongMixer, sample::AudioBuffer};
use super::Parser;
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

    #[cfg(feature = "conv")]
    pub fn to_wave<W>(&self, mut writer: W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        use crate::utils::format::WaveFile;

        writer.write_all(&self.mix().to_wave())
    }
}

impl Parser for Vec<Sound> {
    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
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
#[cfg(feature = "conv")]
mod tests {
    use std::cell::LazyCell;

    use super::*;
    use crate::utils::test::*;

    const SOUND: LazyCell<Vec<u8>> = LazyCell::new(|| deflated_file!("97.dat"));

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let (_, sounds) = Vec::<Sound>::parser(())(&SOUND)?;

        let output_dir = PARSED_PATH.join("sound/song");

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
            .try_for_each(|(i, (sound, song))| -> std::io::Result<()> {
                println!("# SONG {i}");

                output_file(output_dir.join(format!("{i:0>2X}.wav")))
                    .and_then(|w| sound.to_wave(w))?;

                output_file(output_dir.join(format!("{i:0>2X}.txt")))
                    .and_then(|mut w| write!(w, "{song:#?}"))
            })?;

        let output_dir = PARSED_PATH.join("sound/effect");

        sounds
            .iter()
            .filter(|s| matches!(s, Sound::Effect(_)))
            .enumerate()
            .try_for_each(|(i, effect)| {
                output_file(output_dir.join(format!("{i:0>2X}.wav")))
                    .and_then(|w| effect.to_wave(w))?;

                Ok(())
            })
    }
}
