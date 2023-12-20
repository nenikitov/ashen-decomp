use std::io::{self, Cursor};

use crate::{
    asset::{pack_info::PackInfo, sound::dat::mixer::Mixer, AssetChunk},
    utils::nom::*,
};
use itertools::Itertools;
use lewton::inside_ogg::OggStreamReader;

use super::mixer::SoundEffect;

#[derive(Debug)]
pub struct TInstrument {
    pub flags: u8,

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

    pub samples: Box<[u8; 96]>,
}

impl AssetChunk for TInstrument {
    fn parse(input: &[u8]) -> Result<Self> {
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

        let (input, samples) = multi::count!(number::le_u8)(input)?;

        Ok((
            input,
            Self {
                flags,
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
                samples: Box::new(samples),
            },
        ))
    }
}

#[derive(Debug)]
pub struct TSample {
    pub flags: u8,
    pub volume: u8,
    pub panning: u8,
    pub align: u8,
    pub finetune: u32,
    pub loop_length: u32,
    pub loop_end: u32,
    pub sample: u32,
}

impl AssetChunk for TSample {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, flags) = number::le_u8(input)?;
        let (input, volume) = number::le_u8(input)?;
        let (input, panning) = number::le_u8(input)?;
        let (input, align) = number::le_u8(input)?;
        let (input, finetune) = number::le_u32(input)?;
        let (input, loop_length) = number::le_u32(input)?;
        let (input, loop_end) = number::le_u32(input)?;
        let (input, sample) = number::le_u32(input)?;

        Ok((
            input,
            Self {
                flags,
                volume,
                panning,
                align,
                finetune,
                loop_length,
                // The game uses offset for `i16`, but it's much more conventient to just use indeces
                loop_end: loop_end / 2,
                sample: sample / 2,
            },
        ))
    }
}

#[derive(Debug)]
pub struct TSampleParsed {
    pub flags: u8,
    pub volume: u8,
    pub panning: u8,
    pub align: u8,
    pub finetune: u32,
    pub loop_length: u32,
    pub data: Vec<i16>,
}

impl TSampleParsed {
    pub fn parse(header: &TSample, sample_data: &[i16]) -> Self {
        Self {
            flags: header.flags,
            volume: header.volume,
            panning: header.panning,
            align: header.align,
            finetune: header.finetune,
            loop_length: header.loop_length,
            data: sample_data.to_vec(),
        }
    }

    pub fn sample_full(&self) -> &[i16] {
        &self.data
    }

    pub fn sample_loop(&self) -> &[i16] {
        &self.data[self.data.len() - 1 - self.loop_length as usize..]
    }
}
