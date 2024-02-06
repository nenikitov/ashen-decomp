use std::rc::Rc;

use bitflags::bitflags;

use super::{finetune::FineTune, t_instrument::*};
use crate::{
    asset::{extension::*, AssetParser},
    utils::nom::*,
};

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum PatternEventNote {
    #[default]
    Off,
    On(FineTune),
}

impl AssetParser<Wildcard> for Option<PatternEventNote> {
    type Output = Self;

    type Context<'ctx> = bool;

    fn parser(should_parse: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, note) = number::le_u8(input)?;

            Ok((
                input,
                should_parse.then(|| {
                    match note {
                        1..=95 => PatternEventNote::On(FineTune::new_from_note(note as i32)),
                        96 => PatternEventNote::Off,
                        // TODO(nenikitov): Should be a `Result`
                        _ => unreachable!("Note should be in range 0-96"),
                    }
                }),
            ))
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct PatternEventFlags: u8 {
        const ChangeNote = 1 << 0;
        const ChangeInstrument = 1 << 1;
        const ChangeVolume = 1 << 2;
        const ChangeEffect1 = 1 << 3;
        const ChangeEffect2 = 1 << 4;
        const IsEmpty = 1 << 5;
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

impl AssetParser<Wildcard> for Option<PatternEffect> {
    type Output = Self;

    type Context<'ctx> = bool;

    fn parser(should_parse: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, kind) = number::le_u8(input)?;
            let (input, value) = number::le_u8(input)?;

            Ok((
                input,
                should_parse.then(|| PatternEffect {
                    kind: kind.into(),
                    value,
                }),
            ))
        }
    }
}

#[derive(Debug)]
pub enum PatternEventInstrumentKind {
    // TODO(nenikitov): Figure out what instrument `255` is
    Special,
    Predefined(Rc<TInstrument>),
}

impl AssetParser<Wildcard> for Option<PatternEventInstrumentKind> {
    type Output = Self;

    type Context<'ctx> = (bool, &'ctx [Rc<TInstrument>]);

    fn parser(
        (should_parse, instruments): Self::Context<'_>,
    ) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, instrument) = number::le_u8(input)?;

            Ok((
                input,
                should_parse.then(|| {
                    if instrument == 255 {
                        PatternEventInstrumentKind::Special
                    } else {
                        PatternEventInstrumentKind::Predefined(
                            instruments[instrument as usize].clone(),
                        )
                    }
                }),
            ))
        }
    }
}

#[derive(Default)]
pub struct PatternEvent {
    pub note: Option<PatternEventNote>,
    pub instrument: Option<PatternEventInstrumentKind>,
    pub volume: Option<u8>,
    pub effects: [Option<PatternEffect>; 2],
}

impl AssetParser<Wildcard> for PatternEvent {
    type Output = Self;

    type Context<'ctx> = &'ctx [Rc<TInstrument>];

    fn parser(instruments: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, flags) = PatternEventFlags::parser(())(input)?;

            if flags.contains(PatternEventFlags::IsEmpty) {
                Ok((input, Self::default()))
            } else {
                let (input, note) = <Option<PatternEventNote>>::parser(
                    flags.contains(PatternEventFlags::ChangeNote),
                )(input)?;

                let (input, instrument) = <Option<PatternEventInstrumentKind>>::parser((
                    (flags.contains(PatternEventFlags::ChangeInstrument)),
                    instruments,
                ))(input)?;

                let (input, volume) = number::le_u8(input)?;
                let volume = flags
                    .contains(PatternEventFlags::ChangeVolume)
                    .then_some(volume);

                let (input, effect_1) = <Option<PatternEffect>>::parser(
                    flags.contains(PatternEventFlags::ChangeEffect1),
                )(input)?;

                let (input, effect_2) = <Option<PatternEffect>>::parser(
                    flags.contains(PatternEventFlags::ChangeEffect2),
                )(input)?;

                Ok((
                    input,
                    Self {
                        note,
                        instrument,
                        volume,
                        effects: [effect_1, effect_2],
                    },
                ))
            }
        }
    }
}
