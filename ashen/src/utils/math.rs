use num::{Bounded, NumCast, Zero};

pub trait IntoFromNormalizedF32 {
    fn into_normalized_f32(&self) -> f32;
    fn from_normalized_f32(value: f32) -> Self;
}

impl<T> IntoFromNormalizedF32 for T
where
    T: Bounded + NumCast + Zero + PartialOrd,
{
    fn into_normalized_f32(&self) -> f32 {
        if *self >= T::zero() {
            self.to_f32().unwrap() / T::max_value().to_f32().unwrap()
        } else {
            -(self.to_f32().unwrap() / T::min_value().to_f32().unwrap())
        }
    }

    fn from_normalized_f32(value: f32) -> Self {
        if value >= 0.0 {
            T::from((value * T::max_value().to_f32().unwrap()).floor()).unwrap()
        } else {
            T::from((-value * T::min_value().to_f32().unwrap()).floor()).unwrap()
        }
    }
}
