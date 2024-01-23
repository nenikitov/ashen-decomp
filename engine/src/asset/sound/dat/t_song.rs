use super::{mixer::SoundEffect, t_instrument::*, uncompress};
use crate::{
    asset::{extension::*, sound::dat::mixer::Mixer, AssetParser},
    utils::nom::*,
};
use itertools::Itertools;

#[derive(Debug)]
pub struct TSong {
    bpm: u8,
    speed: u8,
    restart_order: u8,
    orders: Vec<u8>,
    /// Reusable and repeatable sequence -> Row -> Channel (`None` to play nothing)
    patterns: Vec<Vec<Vec<Option<TPattern>>>>,
    instruments: Vec<TInstrument>,
    samples: Vec<TSample>,
}

impl TSong {
    pub fn mix(&self) -> Vec<i16> {
        let mut m = Mixer::new();

        let mut i = 0;
        for pattern in &self.orders {
            let pattern = &self.patterns[*pattern as usize];
            for row in pattern {
                i += 1;
                for event in row {
                    if let Some(entry) = event
                        && let Some(note) = entry.note
                        // TODO(nenikitov): Find out what special instrument `0xFF` means
                        && entry.instrument != 0xFF
                    {
                        let instrument = &self.instruments[entry.instrument as usize];
                        // TODO(nenikitov): See if doing `- 1` in parsing will look nicer
                        let sample = instrument.samples[(note - 1) as usize];
                        // TODO(nenikitov): Find out what special sample `0xFF` means
                        if sample != 0xFF {
                            let sample = &self.samples[sample as usize];
                            let data = sample.data.clone();

                            m.add_samples(&data.volume(Self::volume(sample.volume)), i * 1000);
                        }
                    }
                }
            }
        }

        m.mix()
    }

    fn volume(volume: u8) -> f32 {
        volume as f32 / u8::MAX as f32
    }
}

impl AssetParser<Wildcard> for TSong {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
        move |input| {
            let (header, pointers) = {
                let (input, header) = TSongHeader::parser(())(input)?;
                let (input, pointers) = TSongPointers::parser(())(input)?;
                (header, pointers)
            };

            let (_, orders) = multi::count!(number::le_u8, header.song_length as usize)(
                &input[pointers.orders as usize..],
            )?;

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
                        <Option<TPattern>>::parser(()),
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

            let (_, instruments) = multi::count!(
                TInstrument::parser(()),
                header.instrument_count as usize
            )(&input[pointers.instruments as usize..])?;

            let samples = uncompress(&input[pointers.sample_data as usize..]);
            let (_, samples) = multi::count!(
                TSample::parser(&samples),
                header.sample_count as usize
            )(&input[pointers.samples as usize..])?;

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

    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
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

    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
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

#[derive(Debug)]
struct TPattern {
    flags: u8,
    note: Option<u8>,
    instrument: u8,
    volume: u8,
    effect_1: u16,
    effect_2: u16,
}

impl AssetParser<Wildcard> for TPattern {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
        move |input| {
            let (input, flags) = number::le_u8(input)?;
            let (input, note) = number::le_u8(input)?;
            let (input, instrument) = number::le_u8(input)?;
            let (input, volume) = number::le_u8(input)?;
            let (input, effect_1) = number::le_u16(input)?;
            let (input, effect_2) = number::le_u16(input)?;

            Ok((
                input,
                Self {
                    flags,
                    note: (note != 0).then_some(note),
                    instrument,
                    volume,
                    effect_1,
                    effect_2,
                },
            ))
        }
    }
}

impl AssetParser<Wildcard> for Option<TPattern> {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
        move |input| {
            let (after_flags, flags) = number::le_u8(input)?;
            if (flags & 0x20) != 0 {
                Ok((after_flags, None))
            } else {
                let (input, pattern) = TPattern::parser(())(input)?;
                Ok((input, Some(pattern)))
            }
        }
    }
}
