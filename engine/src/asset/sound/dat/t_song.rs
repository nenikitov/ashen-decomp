use std::rc::Rc;

use itertools::Itertools;

use super::{pattern_event::*, t_instrument::*, uncompress};
use crate::{
    asset::{extension::*, AssetParser},
    utils::nom::*,
};

pub type PatternRow = Vec<PatternEvent>;
pub type Pattern = Vec<PatternRow>;

pub struct TSong {
    pub bpm: u8,
    pub speed: u8,
    pub restart_order: u8,
    pub orders: Vec<Rc<Pattern>>,
    /// Reusable and repeatable sequence -> Row -> Channel
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
                        PatternEvent::parser(&instruments),
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
