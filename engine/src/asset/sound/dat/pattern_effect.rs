use std::hash::Hash;

use super::convert_volume;
use crate::{
    asset::{extension::*, AssetParser},
    utils::nom::*,
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum PatternEffectMemoryKey {
    VolumeValue,
    VolumeSlide,
    SampleOffset,
}

#[derive(Debug)]
pub enum Speed {
    TicksPerRow(usize),
    Bpm(usize),
}

#[derive(Debug)]
pub enum Volume {
    Value(f32),
    Slide(Option<f32>),
}

#[derive(Debug)]
pub enum PatternEffect {
    Dummy,
    Speed(Speed),
    Volume(Volume),
    SampleOffset(usize),
}

impl PatternEffect {
    fn memory_key(&self) -> Option<PatternEffectMemoryKey> {
        match self {
            PatternEffect::Volume(Volume::Value(_)) => Some(PatternEffectMemoryKey::VolumeValue),
            PatternEffect::Volume(Volume::Slide(_)) => Some(PatternEffectMemoryKey::VolumeSlide),
            PatternEffect::SampleOffset(_) => Some(PatternEffectMemoryKey::SampleOffset),
            _ => None,
        }
    }
}

impl Hash for PatternEffect {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.memory_key().hash(state)
    }
}

impl AssetParser<Wildcard> for Option<PatternEffect> {
    type Output = Self;

    type Context<'ctx> = bool;

    fn parser(should_parse: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, kind) = number::le_u8(input)?;
            let (input, value) = number::le_u8(input)?;

            Ok((
                input,
                should_parse.then(|| match kind {
                    0x09 => PatternEffect::SampleOffset(value as usize * 256),
                    0x0E => PatternEffect::Speed(if value >= 0x20 {
                        Speed::Bpm(value as usize)
                    } else {
                        Speed::TicksPerRow(value as usize)
                    }),
                    0x0C => PatternEffect::Volume(Volume::Value(convert_volume(value))),
                    0x0A => PatternEffect::Volume(Volume::Slide((value != 0).then(|| {
                        if value >= 16 {
                            convert_volume(value / 16)
                        } else {
                            -convert_volume(value)
                        }
                    }))),
                    // TODO(nenikitov): Remove dummy effect
                    0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x0A | 0x0B
                    | 0x0C | 0x0D | 0x0F | 0x14 | 0x15 | 0x16 | 0x1D | 0x1E | 0x1F | 0x20
                    | 0x21 | 0x22 | 0x24 | 0x25 | 0x2E | 0x2F | 0x30 | 0x31 | 0x32 | 0x33
                    | 0x34 | 0x35 | 0x36 | 0x37 => PatternEffect::Dummy,
                    // TODO(nenikitov): Add support for other effects
                    // 0x00 => Self::Arpegio,
                    // 0x01 => Self::PortaUp,
                    // 0x02 => Self::PortaDown,
                    // 0x03 => Self::PortaTone,
                    // 0x04 => Self::Vibrato,
                    // 0x05 => Self::PortaVolume,
                    // 0x06 => Self::VibratoVolume,
                    // 0x07 => Self::Tremolo,
                    // 0x08 => Self::Pan,
                    // 0x0A => Self::VolumeSlide,
                    // 0x0B => Self::PositionJump,
                    // 0x0C => Self::Volume,
                    // 0x0D => Self::Break,
                    // 0x0F => Self::VolumeGlobal,
                    // 0x14 => Self::Sync,
                    // 0x15 => Self::PortaFineUp,
                    // 0x16 => Self::PortaFineDown,
                    // 0x1D => Self::NoteRetrigger,
                    // 0x1E => Self::VolumeSlideFineUp,
                    // 0x1F => Self::VolumeSlideFineDown,
                    // 0x20 => Self::NoteCut,
                    // 0x21 => ???,
                    // 0x22 => Self::PatternDelay,
                    // 0x24 => Self::PortaExtraFineUp,
                    // 0x25 => Self::PortaExtraFineDown,
                    // 0x2E => Self::SoundControlSurroundOn,
                    // 0x2F => Self::SoundControlSurroundOff,
                    // 0x30 => Self::SoundControlReverbOn,
                    // 0x31 => Self::SoundControlReverbOff,
                    // 0x32 => Self::SoundControlCentre,
                    // 0x33 => Self::SoundControlQuad,
                    // 0x34 => Self::FilterGlobal,
                    // 0x35 => Self::FilterLocal,
                    // 0x36 => Self::PlayForward,
                    // 0x37 => Self::PlayBackward,
                    // TODO(nenikitov): Should be a `Result`
                    kind => unreachable!("Effect is outside the range {kind}"),
                }),
            ))
        }
    }
}
