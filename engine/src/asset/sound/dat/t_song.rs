use super::{t_instrument::*, uncompress};
use crate::{
    asset::{extension::*, AssetParser},
    utils::nom::*,
};
use bitflags::bitflags;
use itertools::Itertools;
use std::rc::Rc;

pub type PatternRow = Vec<Option<PatternEvent>>;
pub type Pattern = Vec<PatternRow>;

#[derive(Debug)]
pub struct TSong {
    pub bpm: u8,
    pub speed: u8,
    pub restart_order: u8,
    pub orders: Vec<Rc<Pattern>>,
    /// Reusable and repeatable sequence -> Row -> Channel (`None` to play nothing)
    pub patterns: Vec<Rc<Pattern>>,
    pub instruments: Vec<Rc<TInstrument>>,
    pub samples: Vec<Rc<TSample>>,
}

impl AssetParser<Wildcard> for TSong {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (header, pointers) = {
                let (input, header) = TSongHeader::parser(())(input)?;
                let (input, pointers) = TSongPointers::parser(())(input)?;
                (header, pointers)
            };

            let samples = uncompress(&input[pointers.sample_data as usize..]);
            let (_, samples) = multi::count!(
                TSample::parser(&samples),
                header.sample_count as usize
            )(&input[pointers.samples as usize..])?;
            let samples = samples.into_iter().map(Rc::new).collect::<Vec<_>>();

            let (_, instruments) = multi::count!(
                TInstrument::parser(&samples),
                header.instrument_count as usize
            )(&input[pointers.instruments as usize..])?;
            let instruments = instruments.into_iter().map(Rc::new).collect::<Vec<_>>();

            let patterns: Vec<_> = {
                let (_, lengths) = multi::count!(number::le_u8, header.pattern_count as usize)(
                    &input[pointers.pattern_lengths as usize..],
                )?;

                multi::count!(number::le_u32, header.pattern_count as usize)(
                    &input[pointers.patterns as usize..],
                )?
                .1
                .into_iter()
                .map(|position| position + pointers.pattern_data)
                .map(|position| &input[position as usize..])
                .zip(lengths)
                .map(|(input, length)| {
                    multi::count!(
                        <Option<PatternEvent>>::parser(&instruments),
                        header.channel_count as usize * length as usize
                    )(input)
                })
                .map(|patterns| patterns.map(|(_, p)| p))
                .map(|patterns| {
                    patterns.map(|p| -> Vec<Vec<_>> {
                        p.into_iter()
                            .chunks(header.channel_count as usize)
                            .into_iter()
                            .map(Iterator::collect)
                            .collect()
                    })
                })
                .collect::<std::result::Result<_, _>>()?
            };
            let patterns = patterns.into_iter().map(Rc::new).collect::<Vec<_>>();

            let (_, orders) = multi::count!(number::le_u8, header.song_length as usize)(
                &input[pointers.orders as usize..],
            )?;
            let orders = orders
                .into_iter()
                .map(|o| patterns[o as usize].clone())
                .collect::<Vec<_>>();

            Ok((
                input,
                Self {
                    bpm: header.bpm,
                    speed: header.speed,
                    restart_order: header.restart_order,
                    orders,
                    patterns,
                    instruments,
                    samples,
                },
            ))
        }
    }
}

#[derive(Debug)]
struct TSongHeader {
    song_length: u8,
    restart_order: u8,
    channel_count: u8,
    pattern_count: u8,
    instrument_count: u8,
    sample_count: u8,
    speed: u8,
    bpm: u8,
}

impl AssetParser<Wildcard> for TSongHeader {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, song_length) = number::le_u8(input)?;
            let (input, restart_order) = number::le_u8(input)?;
            let (input, channel_count) = number::le_u8(input)?;
            let (input, pattern_count) = number::le_u8(input)?;
            let (input, instrument_count) = number::le_u8(input)?;
            let (input, sample_count) = number::le_u8(input)?;
            let (input, speed) = number::le_u8(input)?;
            let (input, bpm) = number::le_u8(input)?;

            Ok((
                input,
                Self {
                    song_length,
                    restart_order,
                    channel_count,
                    pattern_count,
                    instrument_count,
                    sample_count,
                    speed,
                    bpm,
                },
            ))
        }
    }
}

#[derive(Debug)]
struct TSongPointers {
    orders: u32,
    patterns: u32,
    pattern_lengths: u32,
    pattern_data: u32,
    instruments: u32,
    samples: u32,
    sample_data: u32,
}

impl AssetParser<Wildcard> for TSongPointers {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, orders) = number::le_u32(input)?;
            let (input, patterns) = number::le_u32(input)?;
            let (input, pattern_lengths) = number::le_u32(input)?;
            let (input, pattern_data) = number::le_u32(input)?;
            let (input, instruments) = number::le_u32(input)?;
            let (input, samples) = number::le_u32(input)?;
            let (input, sample_data) = number::le_u32(input)?;

            Ok((
                input,
                Self {
                    orders,
                    patterns,
                    pattern_lengths,
                    pattern_data,
                    instruments,
                    samples,
                    sample_data,
                },
            ))
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum NoteState {
    #[default]
    None,
    On(u8),
    Off,
}

impl From<u8> for NoteState {
    fn from(value: u8) -> Self {
        match value {
            0 => NoteState::None,
            1..=95 => NoteState::On(value),
            96 => NoteState::Off,
            _ => unreachable!("Note should be in range 0-96"),
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TPatternFlags: u8 {
        const ChangeNote = 1 << 0;
        const ChangeInstrument = 1 << 1;
        const ChangeVolume = 1 << 2;
        const DoEffect1 = 1 << 3;
        const DoEffect2 = 1 << 4;
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

#[derive(Debug)]
pub struct PatternEvent {
    pub flags: TPatternFlags,
    pub note: NoteState,
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
            let (input, flags) = number::le_u8(input)?;
            let (input, note) = number::le_u8(input)?;
            let (input, instrument_index) = number::le_u8(input)?;
            let (input, volume) = number::le_u8(input)?;
            let (input, effect_1) = PatternEffect::parser(())(input)?;
            let (input, effect_2) = PatternEffect::parser(())(input)?;

            Ok((
                input,
                Self {
                    // TODO(nenikitov): Use `Result`
                    flags: TPatternFlags::from_bits(flags).expect("Flags should be valid"),
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
