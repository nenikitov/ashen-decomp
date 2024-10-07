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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
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
    Speed(Speed),
    Volume(Volume),
    Porta(Porta),
    SampleOffset(Option<usize>),
    PlaybackDirection(PlaybackDirection),
    GlobalVolume(f32),
    NoteDelay(usize),
    PatternBreak,
    PatternJump(usize),
    RetriggerNote(usize),
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

            use PatternEffect as E;
            Ok((
                input,
                should_parse.then(|| match kind {
                    0x01 => E::Porta(Porta::Slide {
                        up: true,
                        finetune: (value != 0).then_some(FineTune::new(8 * value as i32)),
                    }),
                    0x02 => E::Porta(Porta::Slide {
                        up: false,
                        finetune: (value != 0).then_some(-FineTune::new(8 * value as i32)),
                    }),
                    0x03 => E::Porta(Porta::Tone(
                        (value != 0).then_some(FineTune::new(8 * value as i32)),
                    )),
                    0x15 => E::Porta(Porta::Bump {
                        up: true,
                        small: false,
                        finetune: (value != 0).then_some(FineTune::new(8 * value as i32)),
                    }),
                    0x16 => E::Porta(Porta::Bump {
                        up: false,
                        small: false,
                        finetune: (value != 0).then_some(-FineTune::new(8 * value as i32)),
                    }),
                    0x24 => E::Porta(Porta::Bump {
                        up: true,
                        small: true,
                        finetune: (value != 0).then_some(FineTune::new(2 * (value & 0xF) as i32)),
                    }),
                    0x25 => E::Porta(Porta::Bump {
                        up: false,
                        small: true,
                        finetune: (value != 0).then_some(-FineTune::new(2 * (value & 0xF) as i32)),
                    }),
                    0x09 => E::SampleOffset((value != 0).then_some(value as usize * 256)),
                    0x0E => E::Speed(if value >= 0x20 {
                        Speed::Bpm(value as usize)
                    } else {
                        Speed::TicksPerRow(value as usize)
                    }),
                    0x0D => E::PatternBreak,
                    0x0B => E::PatternJump(value as usize),
                    0x1D => E::RetriggerNote(value as usize),
                    0x0C => E::Volume(Volume::Set(convert_volume(value))),
                    0x0A => E::Volume(Volume::Slide((value != 0).then_some(if value >= 16 {
                        convert_volume(value / 16)
                    } else {
                        -convert_volume(value)
                    }))),
                    0x1E => E::Volume(Volume::Bump {
                        up: true,
                        volume: (value != 0).then_some(convert_volume(value)),
                    }),
                    0x1F => E::Volume(Volume::Bump {
                        up: false,
                        volume: (value != 0).then_some(-convert_volume(value)),
                    }),
                    0x0F => E::GlobalVolume(convert_volume(value)),
                    0x21 => E::NoteDelay(value as usize),
                    0x36 => E::PlaybackDirection(PlaybackDirection::Forwards),
                    0x37 => E::PlaybackDirection(PlaybackDirection::Backwards),
                    // TODO(nenikitov): Should be a `Result`
                    0x0..=0x37 => todo!("Ashen effect 0x{kind:X} should have been implemented"),
                    _ => unreachable!("Effect is outside the range 0x{kind:X}"),
                }),
            ))
        }
    }
}
