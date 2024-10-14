use std::{cmp, rc::Rc};

use bitflags::bitflags;

use super::{convert_volume, finetune::FineTune};
use crate::{
    asset::{sound::sample::AudioBuffer, AssetParser},
    utils::{iterator::CollectArray, nom::*},
};

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TInstrumentFlags: u8 {
        const HasVolumeEnvelope = 1 << 0;
        const HasPanEnvelope    = 1 << 1;
    }
}

impl AssetParser for TInstrumentFlags {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, flags) = number::le_u8(input)?;

            Ok((
                input,
                // TODO(nenikitov): Should be a `Result`
                TInstrumentFlags::from_bits(flags).unwrap_or_else(|| {
                    panic!("PatternEvent flags should be valid: received: {flags:b}")
                }),
            ))
        }
    }
}

#[derive(Debug)]
pub struct TInstrumentVolumeEnvelope {
    pub data: Vec<f32>,
    pub sustain: Option<usize>,
}

impl TInstrumentVolumeEnvelope {
    pub fn volume_start(&self) -> &[f32] {
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

impl AssetParser for Option<TInstrumentVolumeEnvelope> {
    type Output = Self;

    type Context<'ctx> = bool;

    fn parser(has_envelope: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, begin) = number::le_u16(input)?;
            let (input, end) = number::le_u16(input)?;
            let (input, sustain) = number::le_u16(input)?;
            let (input, end_total) = number::le_u16(input)?;
            let (input, data) = multi::count!(number::le_u8, TInstrument::ENVELOPE_SIZE)(input)?;

            Ok((
                input,
                has_envelope.then(|| {
                    let data = data
                        .into_iter()
                        .skip(begin as usize)
                        .take(
                            cmp::min(cmp::min(end, end_total), TInstrument::ENVELOPE_SIZE as u16)
                                as usize,
                        )
                        .map(convert_volume)
                        .collect();
                    TInstrumentVolumeEnvelope {
                        data,
                        sustain: (sustain != u16::MAX).then_some((sustain - begin) as usize),
                    }
                }),
            ))
        }
    }
}

#[derive(Debug)]
pub struct TInstrument {
    pub flags: TInstrumentFlags,

    pub volume: Option<TInstrumentVolumeEnvelope>,

    pub pan_begin: u16,
    pub pan_end: u16,
    pub pan_sustain: u16,
    pub pan_envelope_border: u16,
    pub pan_envelope: Box<[u8; Self::ENVELOPE_SIZE]>,

    pub vibrato_depth: u8,
    pub vibrato_speed: u8,
    pub vibrato_sweep: u8,

    pub fadeout: u32,
    pub vibrato_table: u32,

    pub samples: Box<[Option<Rc<TSample>>; 96]>,
}

impl TInstrument {
    const ENVELOPE_SIZE: usize = 325;
}

impl AssetParser for TInstrument {
    type Output = Self;

    type Context<'ctx> = &'ctx [Rc<TSample>];

    fn parser(samples: Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
        move |input| {
            let (input, flags) = TInstrumentFlags::parser(())(input)?;

            let (input, _) = bytes::take(1usize)(input)?;

            let (input, volume_envelope) = Option::<TInstrumentVolumeEnvelope>::parser(
                flags.contains(TInstrumentFlags::HasVolumeEnvelope),
            )(input)?;

            // TODO(nenikitov): None of the instruments (except some weird one in the very first song I believe) use pan
            // See if this is needed
            let (input, pan_begin) = number::le_u16(input)?;
            let (input, pan_end) = number::le_u16(input)?;
            let (input, pan_sustain) = number::le_u16(input)?;
            let (input, pan_envelope_border) = number::le_u16(input)?;
            let (input, pan_envelope) = multi::count!(number::le_u8)(input)?;

            let (input, _) = bytes::take(1usize)(input)?;

            // TODO(nenikitov): None of the instruments use vibrato
            // See if this is needed
            let (input, vibrato_depth) = number::le_u8(input)?;
            let (input, vibrato_speed) = number::le_u8(input)?;
            let (input, vibrato_sweep) = number::le_u8(input)?;

            // TODO(nenikitov): There is some variation (0, 256, 1024)
            // But it's only used by effects?
            let (input, fadeout) = number::le_u32(input)?;
            // TODO(nenikitov): There is some variation (0, 256)
            // But it's only used by effects?
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
                            .map(|i| samples.get(i as usize).map(Rc::clone))
                            .collect_array(),
                    ),
                },
            ))
        }
    }
}

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
    pub loop_len: usize,
    pub buffer: AudioBuffer<i16>,
}

impl AssetParser for TSample {
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
            let loop_length = loop_length / 2;

            Ok((
                input,
                Self {
                    flags: TSampleFlags::from_bits(flags).expect("Flags should be valid"),
                    volume: convert_volume(volume),
                    panning,
                    align,
                    finetune: FineTune::new(finetune),
                    loop_len: loop_length as usize,
                    buffer: AudioBuffer {
                        data: sample_data[sample_offset as usize..loop_end as usize].to_vec(),
                        sample_rate: Self::SAMPLE_RATE,
                    },
                },
            ))
        }
    }
}

impl TSample {
    pub const SAMPLE_RATE: usize = 16_000;

    // TODO(nenikitov): I think the whole `Sample` will need to be removed
    pub fn get(&self, position: f64) -> Option<i16> {
        if position < 0.0 {
            return None;
        }

        let position = Self::SAMPLE_RATE as f64 * position;

        let frac = position.fract() as f32;

        let prev = self.normalize(position as usize)?;
        let prev = self.buffer[prev] as f32;

        let next = self.normalize(position as usize + 1);
        let next = next.map_or(0.0, |next| self.buffer[next] as f32);

        Some(frac.mul_add(next - prev, prev) as i16)
    }

    fn normalize(&self, position: usize) -> Option<usize> {
        if position < self.buffer.len_samples() {
            Some(position)
        } else if self.loop_len == 0 {
            None
        } else {
            Some(
                self.buffer.len_samples() - self.loop_len
                    + (position - self.buffer.len_samples()) % self.loop_len,
            )
        }
    }
}
