use super::{
    mixer::Mixer,
    t_instrument::{TInstrument, TSample},
    uncompress,
};
use crate::{
    asset::{extension::*, AssetParser},
    utils::nom::*,
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

impl AssetParser<Wildcard> for TEffect {
    type Output = Self;

    type Context<'ctx> = ();

    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
        move |input| {
            let (_, pointers) = TEffectPointers::parser(())(input)?;

            let (_, instrument) = TInstrument::parser(())(&input[pointers.instrument as usize..])?;

            let sample = uncompress(&input[pointers.sample_data as usize..]);
            let (_, sample) = TSample::parser(&sample)(&input[pointers.sample as usize..])?;

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

    fn parser((): Self::Context<'_>) -> impl FnParser<Self::Output> {
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
