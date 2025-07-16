use std::rc::Rc;

use super::{
    t_instrument::{TInstrument, TSample},
    uncompress,
};
use crate::{
    asset::{Parser, sound::sample::AudioBuffer},
    utils::nom::*,
};

pub struct TEffect {
    instrument: TInstrument,
    sample: Rc<TSample>,
}

impl TEffect {
    pub fn mix(&self) -> AudioBuffer<i16> {
        self.sample.buffer.clone()
    }
}

impl Parser for TEffect {
    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
        move |input| {
            let (_, pointers) = TEffectPointers::parser(())(input)?;

            let sample = uncompress(&input[pointers.sample_data as usize..]);
            let (_, sample) = TSample::parser(&sample)(&input[pointers.sample as usize..])?;
            let sample = [Rc::new(sample)];

            let (_, instrument) =
                TInstrument::parser(&sample)(&input[pointers.instrument as usize..])?;

            let [sample] = sample;

            Ok((&[], Self { instrument, sample }))
        }
    }
}

#[derive(Debug)]
struct TEffectPointers {
    instrument: u32,
    sample: u32,
    sample_data: u32,
}

impl Parser for TEffectPointers {
    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self> {
        move |input| {
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
}
