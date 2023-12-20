use std::io;

use crate::{
    asset::{sound::dat::t_instrument::TSampleParsed, AssetChunk},
    utils::nom::*,
};

use super::{
    mixer::Mixer,
    t_instrument::{TInstrument, TSample},
    uncompress,
};

pub struct TEffect {
    instrument: TInstrument,
    sample: TSampleParsed,
}

// It should be separated
impl TEffect {
    pub fn mix(&self) -> Vec<i16> {
        let mut m = Mixer::new();

        m.add_samples(&self.sample.sample, 0);

        m.mix()
    }
}

impl AssetChunk for TEffect {
    fn parse(input: &[u8]) -> crate::utils::nom::Result<Self> {
        let (_, pointers) = TEffectPointers::parse(input)?;

        let (_, instrument) = TInstrument::parse(&input[pointers.instrument as usize..])?;

        let sample = {
            let data = uncompress(&input[pointers.sample_data as usize..]);
            let (_, sample) = TSample::parse(&input[pointers.sample as usize..])?;

            TSampleParsed::parse(
                &sample,
                &data[sample.sample as usize..sample.loop_end as usize],
            )
        };

        Ok((&[], Self { instrument, sample }))
    }
}

#[derive(Debug)]
struct TEffectPointers {
    instrument: u32,
    sample: u32,
    sample_data: u32,
}

impl AssetChunk for TEffectPointers {
    fn parse(input: &[u8]) -> crate::utils::nom::Result<Self> {
        let (input, instrument) = number::le_u32(input)?;
        let (input, sample) = number::le_u32(input)?;
        let (input, sample_data) = number::le_u32(input)?;

        Ok((
            input,
            Self {
                instrument,
                sample,
                sample_data,
            },
        ))
    }
}
