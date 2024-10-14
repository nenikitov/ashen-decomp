use std::rc::Rc;

use super::{
    t_instrument::{TInstrument, TSample},
    uncompress,
};
use crate::{
    asset::{extension::*, sound::sample::AudioBuffer, AssetParser},
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

impl AssetParser<Wildcard> for TEffect {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
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

impl AssetParser<Wildcard> for TEffectPointers {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl Fn(Input) -> Result<Self::Output> {
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
