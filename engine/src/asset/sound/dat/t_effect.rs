use crate::{
    asset::{AssetChunk, AssetChunkWithContext},
    utils::nom::*,
};

use super::{
    mixer::Mixer,
    t_instrument::{TInstrument, TSample},
    uncompress,
};

pub struct TEffect {
    instrument: TInstrument,
    sample: TSample,
}

// It should be separated
impl TEffect {
    pub fn mix(&self) -> Vec<i16> {
        let mut m = Mixer::new();
        m.add_samples(&self.sample.data, 0);
        m.mix()
    }
}

impl AssetChunk for TEffect {
    fn parse(input: &[u8]) -> crate::utils::nom::Result<Self> {
        let (_, pointers) = TEffectPointers::parse(input)?;

        let (_, instrument) = TInstrument::parse(&input[pointers.instrument as usize..])?;

        let sample = uncompress(&input[pointers.sample_data as usize..]);
        let (_, sample) = TSample::parse(&sample)(&input[pointers.sample as usize..])?;

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
