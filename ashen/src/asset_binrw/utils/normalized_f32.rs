use num::{Bounded, NumCast, Zero};

use super::*;
use crate::utils::math::IntoFromNormalizedF32;

#[binrw]
#[derive(Debug)]
pub struct NormalizedF32<T>(T)
where
    T: Bounded
        + NumCast
        + Zero
        + PartialOrd
        + Ord
        + Copy
        + for<'a> BinRead<Args<'a> = ()>
        + for<'a> BinWrite<Args<'a> = ()>;

impl<T> From<f32> for NormalizedF32<T>
where
    T: Bounded
        + NumCast
        + Zero
        + PartialOrd
        + Ord
        + Copy
        + for<'a> BinRead<Args<'a> = ()>
        + for<'a> BinWrite<Args<'a> = ()>,
{
    fn from(value: f32) -> Self {
        Self(T::from_normalized_f32(value))
    }
}

impl<T> From<NormalizedF32<T>> for f32
where
    T: Bounded
        + NumCast
        + Zero
        + PartialOrd
        + Ord
        + Copy
        + for<'a> BinRead<Args<'a> = ()>
        + for<'a> BinWrite<Args<'a> = ()>,
{
    fn from(value: NormalizedF32<T>) -> Self {
        value.0.into_normalized_f32()
    }
}
