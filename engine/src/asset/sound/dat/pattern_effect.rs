use std::hash::Hash;

use super::{convert_volume, finetune::FineTune};
use crate::{
    asset::{extension::*, AssetParser},
    utils::nom::*,
};

#[derive(Debug, Clone, Copy)]
pub enum Speed {
    TicksPerRow(usize),
    Bpm(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum Porta {
    Tone(Option<FineTune>),
    Slide {
        up: bool,
        finetune: Option<FineTune>,
    },
    Bump {
        up: bool,
        small: bool,
        finetune: Option<FineTune>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Volume {
    Set(f32),
    Slide(Option<f32>),
    Bump { up: bool, volume: Option<f32> },
}

#[derive(Debug, Default, Clone, Copy)]
pub enum PlaybackDirection {
    #[default]
    Forwards,
    Backwards,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum PatternEffectMemoryKey {
    VolumeSlide,
    VolumeBumpUp,
    VolumeBumpDown,
    SampleOffset,
    PortaTone,
    PortaSlideUp,
    PortaSlideDown,
    PortaBumpUp,
    PortaBumpDown,
    PortaBumpSmallUp,
    PortaBumpSmallDown,
}

#[derive(Debug, Clone, Copy)]
pub enum PatternEffect {
    Dummy(u8),
    Speed(Speed),
    Volume(Volume),
    Porta(Porta),
    SampleOffset(Option<usize>),
    PlaybackDirection(PlaybackDirection),
    GlobalVolume(f32),
}

impl PatternEffect {
    pub fn memory_key(&self) -> Option<PatternEffectMemoryKey> {
        match self {
            PatternEffect::Porta(Porta::Tone(..)) => Some(PatternEffectMemoryKey::PortaTone),
            PatternEffect::Porta(Porta::Slide { up: true, .. }) => {
                Some(PatternEffectMemoryKey::PortaSlideUp)
            }
            PatternEffect::Porta(Porta::Slide { up: false, .. }) => {
                Some(PatternEffectMemoryKey::PortaSlideDown)
            }
            PatternEffect::Porta(Porta::Bump {
                up: true,
                small: false,
                ..
            }) => Some(PatternEffectMemoryKey::PortaBumpUp),
            PatternEffect::Porta(Porta::Bump {
                up: false,
                small: false,
                ..
            }) => Some(PatternEffectMemoryKey::PortaBumpDown),
            PatternEffect::Porta(Porta::Bump {
                up: true,
                small: true,
                ..
            }) => Some(PatternEffectMemoryKey::PortaBumpSmallUp),
            PatternEffect::Porta(Porta::Bump {
                up: false,
                small: true,
                ..
            }) => Some(PatternEffectMemoryKey::PortaBumpSmallDown),
            PatternEffect::Volume(Volume::Slide(..)) => Some(PatternEffectMemoryKey::VolumeSlide),
            PatternEffect::Volume(Volume::Bump { up: true, .. }) => {
                Some(PatternEffectMemoryKey::VolumeBumpUp)
            }
            PatternEffect::Volume(Volume::Bump { up: down, .. }) => {
                Some(PatternEffectMemoryKey::VolumeBumpDown)
            }
            PatternEffect::SampleOffset(..) => Some(PatternEffectMemoryKey::SampleOffset),
            _ => None,
        }
    }

    pub fn has_memory(&self) -> bool {
        self.memory_key().is_some()
    }

    pub fn is_empty(&self) -> bool {
        matches!(
            self,
            PatternEffect::Porta(Porta::Tone(None))
                | PatternEffect::Porta(Porta::Slide { finetune: None, .. })
                | PatternEffect::Porta(Porta::Bump { finetune: None, .. })
                | PatternEffect::Volume(Volume::Slide(None))
                | PatternEffect::Volume(Volume::Bump { volume: None, .. })
                | PatternEffect::SampleOffset(None)
        )
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
                should_parse.then_some(match kind {
                    0x01 => PatternEffect::Porta(Porta::Slide {
                        up: true,
                        finetune: (value != 0).then_some(FineTune::new(8 * value as i32)),
                    }),
                    0x02 => PatternEffect::Porta(Porta::Slide {
                        up: false,
                        finetune: (value != 0).then_some(-FineTune::new(8 * value as i32)),
                    }),
                    0x03 => PatternEffect::Porta(Porta::Tone(
                        (value != 0).then_some(FineTune::new(8 * value as i32)),
                    )),
                    0x15 => PatternEffect::Porta(Porta::Bump {
                        up: true,
                        small: false,
                        finetune: (value != 0).then_some(FineTune::new(8 * value as i32)),
                    }),
                    0x16 => PatternEffect::Porta(Porta::Bump {
                        up: false,
                        small: false,
                        finetune: (value != 0).then_some(-FineTune::new(8 * value as i32)),
                    }),
                    0x24 => PatternEffect::Porta(Porta::Bump {
                        up: true,
                        small: true,
                        finetune: (value != 0).then_some(FineTune::new(2 * (value & 0xF) as i32)),
                    }),
                    0x25 => PatternEffect::Porta(Porta::Bump {
                        up: false,
                        small: true,
                        finetune: (value != 0).then_some(-FineTune::new(2 * (value & 0xF) as i32)),
                    }),
                    0x09 => {
                        PatternEffect::SampleOffset((value != 0).then_some(value as usize * 256))
                    }
                    0x0E => PatternEffect::Speed(if value >= 0x20 {
                        Speed::Bpm(value as usize)
                    } else {
                        Speed::TicksPerRow(value as usize)
                    }),
                    0x0C => PatternEffect::Volume(Volume::Set(convert_volume(value))),
                    0x0A => PatternEffect::Volume(Volume::Slide((value != 0).then_some(
                        if value >= 16 {
                            convert_volume(value / 16)
                        } else {
                            -convert_volume(value)
                        },
                    ))),
                    0x0F => PatternEffect::GlobalVolume(convert_volume(value)),
                    // TODO(nenikitov): Remove dummy effect
                    0x00 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08 | 0x0A | 0x0B | 0x0C | 0x0D
                    | 0x14 | 0x15 | 0x16 | 0x1D | 0x1E | 0x1F | 0x20 | 0x21 | 0x22 | 0x2E
                    | 0x2F | 0x30 | 0x31 | 0x32 | 0x33 | 0x34 | 0x35 => PatternEffect::Dummy(kind),
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
                    0x36 => PatternEffect::PlaybackDirection(PlaybackDirection::Forwards),
                    0x37 => PatternEffect::PlaybackDirection(PlaybackDirection::Backwards),
                    // TODO(nenikitov): Should be a `Result`
                    kind => unreachable!("Effect is outside the range {kind}"),
                }),
            ))
        }
    }
}
