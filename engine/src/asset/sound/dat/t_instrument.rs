use std::{cmp, rc::Rc};

use bitflags::bitflags;

use super::{convert_volume, finetune::FineTune};
use crate::{
    asset::{extension::*, sound::sample::Sample, AssetParser},
    utils::{iterator::CollectArray, nom::*},
};

// TODO(nenikitov): Double check these flags
bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TInstrumentFlags: u8 {
        const HasVolumeEnvelope = 1 << 0;
        const HasPanEnvelope = 1 << 1;
    }
}

impl AssetParser<Wildcard> for TInstrumentFlags {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, flags) = number::le_u8(input)?;

            Ok((
                input,
                // TODO(nenikitov): Should be a `Result`
                TInstrumentFlags::from_bits(flags).expect(&format!(
                    "PatternEvent flags should be valid: received: {flags:b}"
                )),
            ))
        }
    }
}

// TODO(nenikitov): Maybe make it an `AssetParser`
#[derive(Debug)]
pub enum TInstrumentSampleKind {
    // TODO(nenikitov): Figure out what sample `255` is
    Special,
    Predefined(Rc<TSample>),
}

#[derive(Debug)]
pub struct TInstrumentVolumeEnvelope {
    data: Vec<f32>,
    sustain: Option<usize>,
}

impl TInstrumentVolumeEnvelope {
    pub fn volume_beginning(&self) -> &[f32] {
        if let Some(sustain) = self.sustain {
            &self.data[0..sustain]
        } else {
            &self.data
        }
    }

    pub fn volume_loop(&self) -> f32 {
        if let Some(sustain) = self.sustain {
            self.data[sustain]
        } else {
            0.0
        }
    }

    pub fn volume_end(&self) -> &[f32] {
        if let Some(sustain) = self.sustain {
            &self.data[sustain + 1..]
        } else {
            &[]
        }
    }
}

#[derive(Debug)]
pub enum TInstrumentVolume {
    Envelope(TInstrumentVolumeEnvelope),
    Constant(f32),
}

impl AssetParser<Wildcard> for TInstrumentVolume {
    type Output = Self;

    type Context<'ctx> = bool;

    fn parser(has_envelope: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, begin) = number::le_u16(input)?;
            let (input, end) = number::le_u16(input)?;
            let (input, sustain) = number::le_u16(input)?;
            let (input, end_total) = number::le_u16(input)?;
            let (input, data) = multi::count!(number::le_u8, 325)(input)?;

            Ok((
                input,
                if has_envelope {
                    let data = data
                        .into_iter()
                        .skip(begin as usize)
                        .take(cmp::min(cmp::min(end, end_total), 325) as usize)
                        .map(convert_volume)
                        .collect::<Vec<_>>();
                    TInstrumentVolume::Envelope(TInstrumentVolumeEnvelope {
                        data,
                        sustain: if sustain == u16::MAX {
                            None
                        } else {
                            Some((sustain - begin) as usize)
                        },
                    })
                } else {
                    TInstrumentVolume::Constant(0.25)
                },
            ))
        }
    }
}

#[derive(Debug)]
pub struct TInstrument {
    pub flags: TInstrumentFlags,

    pub volume: TInstrumentVolume,

    pub pan_begin: u16,
    pub pan_end: u16,
    pub pan_sustain: u16,
    pub pan_envelope_border: u16,
    pub pan_envelope: Box<[u8; 325]>,

    pub vibrato_depth: u8,
    pub vibrato_speed: u8,
    pub vibrato_sweep: u8,

    pub fadeout: u32,
    pub vibrato_table: u32,

    pub samples: Box<[TInstrumentSampleKind; 96]>,
}

impl AssetParser<Wildcard> for TInstrument {
    type Output = Self;

    type Context<'ctx> = &'ctx [Rc<TSample>];

    fn parser(samples: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, flags) = TInstrumentFlags::parser(())(input)?;

            let (input, _) = bytes::take(1usize)(input)?;

            let (input, volume_envelope) = TInstrumentVolume::parser(
                flags.contains(TInstrumentFlags::HasVolumeEnvelope),
            )(input)?;

            let (input, pan_begin) = number::le_u16(input)?;
            let (input, pan_end) = number::le_u16(input)?;
            let (input, pan_sustain) = number::le_u16(input)?;
            let (input, pan_envelope_border) = number::le_u16(input)?;
            let (input, pan_envelope) = multi::count!(number::le_u8)(input)?;

            let (input, _) = bytes::take(1usize)(input)?;

            let (input, vibrato_depth) = number::le_u8(input)?;
            let (input, vibrato_speed) = number::le_u8(input)?;
            let (input, vibrato_sweep) = number::le_u8(input)?;

            let (input, fadeout) = number::le_u32(input)?;
            let (input, vibrato_table) = number::le_u32(input)?;

            let (input, sample_indexes): (_, [_; 96]) = multi::count!(number::le_u8)(input)?;

            Ok((
                input,
                Self {
                    flags,
                    volume: volume_envelope,
                    pan_begin,
                    pan_end,
                    pan_sustain,
                    pan_envelope_border,
                    pan_envelope: Box::new(pan_envelope),
                    vibrato_depth,
                    vibrato_speed,
                    vibrato_sweep,
                    fadeout,
                    vibrato_table,
                    samples: Box::new(
                        sample_indexes
                            .into_iter()
                            .map(|i| {
                                if i == u8::MAX {
                                    TInstrumentSampleKind::Special
                                } else {
                                    TInstrumentSampleKind::Predefined(samples[i as usize].clone())
                                }
                            })
                            .collect_array(),
                    ),
                },
            ))
        }
    }
}

// TODO(nenikitov): I'm not sure about this flag
bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TSampleFlags: u8 {
        const IsLooping = 1 << 0;
    }
}

#[derive(Debug)]
pub struct TSample {
    pub flags: TSampleFlags,
    pub volume: f32,
    pub panning: u8,
    pub align: u8,
    pub finetune: FineTune,
    pub loop_length: f32,
    pub data: Sample<i16, 1>,
}

impl TSample {
    const SAMPLE_RATE: usize = 16_000;
}

impl AssetParser<Wildcard> for TSample {
    type Output = Self;

    type Context<'ctx> = &'ctx [i16];

    fn parser(sample_data: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, flags) = number::le_u8(input)?;
            let (input, volume) = number::le_u8(input)?;
            let (input, panning) = number::le_u8(input)?;
            let (input, align) = number::le_u8(input)?;
            let (input, finetune) = number::le_i32(input)?;
            let (input, loop_length) = number::le_u32(input)?;
            let (input, loop_end) = number::le_u32(input)?;
            let (input, sample_offset) = number::le_u32(input)?;

            // The game uses offset for `i16`, but it's much more convenient to just use time, so that's why `/ 2` (`i16` is 2 bytes)
            let loop_end = loop_end / 2;
            let sample_offset = sample_offset / 2;
            let loop_length = loop_length as f32 / 2.0 / Self::SAMPLE_RATE as f32;

            Ok((
                input,
                Self {
                    flags: TSampleFlags::from_bits(flags).expect("Flags should be valid"),
                    volume: convert_volume(volume),
                    panning,
                    align,
                    finetune: FineTune::new(finetune),
                    // TODO(nenikitov): Look into resampling the sample to 48 KHz
                    loop_length,
                    data: Sample {
                        data: sample_data[sample_offset as usize..loop_end as usize]
                            .into_iter()
                            .map(|&s| [s])
                            .collect(),
                        sample_rate: Self::SAMPLE_RATE,
                    },
                },
            ))
        }
    }
}

impl TSample {
    pub fn sample_beginning(&self) -> &[[i16; 1]] {
        &self.data[..self.data.len_seconds() - self.loop_length]
    }

    pub fn sample_loop(&self) -> &[[i16; 1]] {
        if self.loop_length != 0.0 {
            &self.data[self.data.len_seconds() - self.loop_length..]
        } else {
            &[]
        }
    }
}
