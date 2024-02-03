use super::t_instrument::*;
use crate::{
    asset::{extension::*, AssetParser},
    utils::nom::*,
};
use bitflags::bitflags;
use std::rc::Rc;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum PatternEventNote {
    #[default]
    None,
    On(u8),
    Off,
}

impl AssetParser<Wildcard> for PatternEventNote {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, note) = number::le_u8(input)?;

            let note = match note {
                0 => PatternEventNote::None,
                1..=95 => PatternEventNote::On(note),
                96 => PatternEventNote::Off,
                // TODO(nenikitov): Should be a `Result`
                _ => unreachable!("Note should be in range 0-96"),
            };

            Ok((input, note))
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct PatternEventFlags: u8 {
        const ChangeNote = 1 << 0;
        const ChangeInstrument = 1 << 1;
        const ChangeVolume = 1 << 2;
        const DoEffect1 = 1 << 3;
        const DoEffect2 = 1 << 4;
    }
}

impl AssetParser<Wildcard> for PatternEventFlags {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, flags) = number::le_u8(input)?;

            Ok((
                input,
                // TODO(nenikitov): Should be a `Result`
                PatternEventFlags::from_bits(flags).expect(&format!(
                    "PatternEvent flags should be valid: received: {flags:b}"
                )),
            ))
        }
    }
}

#[derive(Debug)]
pub enum PatternEffectKind {
    None,
    Arpegio,
    PortaUp,
    PortaDown,
    PortaTone,
    Vibrato,
    PortaVolume,
    VibratoVolume,
    Tremolo,
    Pan,
    SampleOffset,
    VolumeSlide,
    PositionJump,
    Volume,
    Break,
    Speed,
    VolumeGlobal,
    Sync,
    PortaFineUp,
    PortaFineDown,
    NoteRetrigger,
    VolumeSlideFineUp,
    VolumeSlideFineDown,
    NoteCut,
    NoteDelay,
    PatternDelay,
    PortaExtraFineUp,
    PortaExtraFineDown,
    // TODO(nenikitov): Verify if it's referring to surround sound
    SoundControlSurroundOff,
    SoundControlSurroundOn,
    SoundControlReverbOn,
    SoundControlReverbOff,
    SoundControlCentre,
    SoundControlQuad,
    FilterGlobal,
    FilterLocal,
    PlayForward,
    PlayBackward,
}

impl From<u8> for PatternEffectKind {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Arpegio,
            0x01 => Self::PortaUp,
            0x02 => Self::PortaDown,
            0x03 => Self::PortaTone,
            0x04 => Self::Vibrato,
            0x05 => Self::PortaVolume,
            0x06 => Self::VibratoVolume,
            0x07 => Self::Tremolo,
            0x08 => Self::Pan,
            0x09 => Self::SampleOffset,
            0x0A => Self::VolumeSlide,
            0x0B => Self::PositionJump,
            0x0C => Self::Volume,
            0x0D => Self::Break,
            0x0E => Self::Speed,
            0x0F => Self::VolumeGlobal,
            0x14 => Self::Sync,
            0x15 => Self::PortaFineUp,
            0x16 => Self::PortaFineDown,
            0x1D => Self::NoteRetrigger,
            0x1E => Self::VolumeSlideFineUp,
            0x1F => Self::VolumeSlideFineDown,
            0x20 => Self::NoteCut,
            0x21 => Self::NoteDelay,
            0x22 => Self::PatternDelay,
            0x24 => Self::PortaExtraFineUp,
            0x25 => Self::PortaExtraFineDown,
            0x2E => Self::SoundControlSurroundOn,
            0x2F => Self::SoundControlSurroundOff,
            0x30 => Self::SoundControlReverbOn,
            0x31 => Self::SoundControlReverbOff,
            0x32 => Self::SoundControlCentre,
            0x33 => Self::SoundControlQuad,
            0x34 => Self::FilterGlobal,
            0x35 => Self::FilterLocal,
            0x35 => Self::FilterLocal,
            0x36 => Self::PlayForward,
            0x37 => Self::PlayBackward,
            _ => Self::None,
        }
    }
}

// TODO(nenikitov): Use enum with associated value instead of a struct
#[derive(Debug)]
pub struct PatternEffect {
    pub kind: PatternEffectKind,
    pub value: u8,
}

impl AssetParser<Wildcard> for PatternEffect {
    type Output = PatternEffect;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, kind) = number::le_u8(input)?;
            let (input, value) = number::le_u8(input)?;

            Ok((
                input,
                Self {
                    kind: kind.into(),
                    value,
                },
            ))
        }
    }
}

#[derive(Debug)]
pub enum TPatternInstrumentKind {
    // TODO(nenikitov): Figure out what instrument `255` is
    Special,
    Predefined(Rc<TInstrument>),
}

pub struct PatternEvent {
    pub flags: PatternEventFlags,
    pub note: PatternEventNote,
    pub instrument: TPatternInstrumentKind,
    pub volume: u8,
    pub effect_1: PatternEffect,
    pub effect_2: PatternEffect,
}

impl AssetParser<Wildcard> for PatternEvent {
    type Output = Self;

    type Context<'ctx> = &'ctx [Rc<TInstrument>];

    fn parser(instruments: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, flags) = PatternEventFlags::parser(())(input)?;
            let (input, note) = PatternEventNote::parser(())(input)?;
            let (input, instrument_index) = number::le_u8(input)?;
            let (input, volume) = number::le_u8(input)?;
            let (input, effect_1) = PatternEffect::parser(())(input)?;
            let (input, effect_2) = PatternEffect::parser(())(input)?;

            Ok((
                input,
                Self {
                    // TODO(nenikitov): Use `Result`
                    flags,
                    note: note.into(),
                    instrument: if instrument_index == 255 {
                        TPatternInstrumentKind::Special
                    } else {
                        TPatternInstrumentKind::Predefined(
                            instruments[instrument_index as usize].clone(),
                        )
                    },
                    volume,
                    effect_1,
                    effect_2,
                },
            ))
        }
    }
}

impl AssetParser<Wildcard> for Option<PatternEvent> {
    type Output = Self;

    type Context<'ctx> = &'ctx [Rc<TInstrument>];

    fn parser(instruments: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (after_flags, flags) = number::le_u8(input)?;
            if (flags & 0x20) != 0 {
                Ok((after_flags, None))
            } else {
                let (input, pattern) = PatternEvent::parser(instruments)(input)?;
                Ok((input, Some(pattern)))
            }
        }
    }
}
