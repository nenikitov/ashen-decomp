use std::rc::Rc;

use crate::{
    asset::{extension::*, AssetParser},
    utils::nom::*,
};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TInstrumentFlags: u8 {
        const _ = 1 << 0;
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
pub struct TInstrument {
    pub flags: TInstrumentFlags,

    pub volume_begin: u16,
    pub volume_end: u16,
    pub volume_sustain: u16,
    pub volume_envelope_border: u16,
    pub volume_envelope: Box<[u8; 325]>,

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
            let (input, flags) = number::le_u8(input)?;

            let (input, _) = bytes::take(1usize)(input)?;

            let (input, volume_begin) = number::le_u16(input)?;
            let (input, volume_end) = number::le_u16(input)?;
            let (input, volume_sustain) = number::le_u16(input)?;
            let (input, volume_envelope_border) = number::le_u16(input)?;
            let (input, volume_envelope) = multi::count!(number::le_u8)(input)?;

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

            let (input, sample_indexes): (_, [u8; 96]) = multi::count!(number::le_u8)(input)?;

            Ok((
                input,
                Self {
                    flags: TInstrumentFlags::from_bits(flags).expect("Flags should be valid"),
                    volume_begin,
                    volume_end,
                    volume_sustain,
                    volume_envelope_border,
                    volume_envelope: Box::new(volume_envelope),
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
                                if i == 255 {
                                    TInstrumentSampleKind::Special
                                } else {
                                    TInstrumentSampleKind::Predefined(samples[i as usize].clone())
                                }
                            })
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap(),
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
    pub volume: u8,
    pub panning: u8,
    pub align: u8,
    pub finetune: u32,
    pub loop_length: u32,
    pub data: Vec<i16>,
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
            let (input, finetune) = number::le_u32(input)?;
            let (input, loop_length) = number::le_u32(input)?;
            let (input, loop_end) = number::le_u32(input)?;
            let (input, sample_offset) = number::le_u32(input)?;

            // The game uses offset for `i16`, but it's much more convenient to just use indices
            let loop_end = loop_end / 2;
            let sample_offset = sample_offset / 2;
            let loop_length = loop_length / 2;

            Ok((
                input,
                Self {
                    flags: TSampleFlags::from_bits(flags).expect("Flags should be valid"),
                    volume,
                    panning,
                    align,
                    finetune,
                    loop_length,
                    data: sample_data[sample_offset as usize..loop_end as usize].to_vec(),
                },
            ))
        }
    }
}

impl TSample {
    pub fn sample_full(&self) -> &[i16] {
        &self.data
    }

    pub fn sample_beginning(&self) -> &[i16] {
        dbg!(self.data.len());
        dbg!(self.loop_length);
        &self.data[..self.data.len() - self.loop_length as usize]
    }

    pub fn sample_loop(&self) -> &[i16] {
        if self.loop_length != 0 {
            &self.data[self.data.len() - self.loop_length as usize..]
        } else {
            &[]
        }
    }
}
