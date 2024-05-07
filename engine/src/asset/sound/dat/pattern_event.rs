use std::rc::Rc;

use bitflags::bitflags;

use super::{convert_volume, finetune::FineTune, pattern_effect::PatternEffect, t_instrument::*};
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
                        1..=95 => PatternEventNote::On(FineTune::from_note(note as i32)),
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

#[derive(Debug, Clone, Copy)]
pub enum PatternEventVolume {
    Sample,
    Value(f32),
}

impl Default for PatternEventVolume {
    fn default() -> Self {
        PatternEventVolume::Value(0.0)
    }
}

impl AssetParser<Wildcard> for Option<PatternEventVolume> {
    type Output = Self;

    type Context<'ctx> = bool;

    fn parser(should_parse: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, volume) = number::le_u8(input)?;

            Ok((
                input,
                should_parse.then(|| {
                    if volume == u8::MAX {
                        PatternEventVolume::Sample
                    } else {
                        PatternEventVolume::Value(convert_volume(volume))
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
    pub volume: Option<PatternEventVolume>,
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

                let (input, volume) = <Option<PatternEventVolume>>::parser(
                    flags.contains(PatternEventFlags::ChangeVolume),
                )(input)?;

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
