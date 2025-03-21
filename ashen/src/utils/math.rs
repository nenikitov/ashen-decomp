use num::{Bounded, NumCast, Zero};

pub trait IntoFromNormalizedF32 {
    fn into_normalized_f32_between(self, min: Self, max: Self, clamp: bool) -> f32;
    fn into_normalized_f32(self) -> f32;
    fn from_normalized_f32_between(value: f32, min: Self, max: Self, clamp: bool) -> Self;
    fn from_normalized_f32(value: f32) -> Self;
}

impl<T> IntoFromNormalizedF32 for T
where
    T: Bounded + NumCast + Zero + PartialOrd + Ord + Copy,
{
    fn into_normalized_f32_between(self, min: Self, max: Self, clamp: bool) -> f32 {
        debug_assert!(min <= T::zero());
        debug_assert!(max > T::zero());

        let value = if clamp { self.clamp(min, max) } else { self };
        if value >= T::zero() {
            value.to_f32().unwrap() / max.to_f32().unwrap()
        } else {
            (-value.to_f32().unwrap() / min.to_f32().unwrap())
        }
    }

    fn into_normalized_f32(self) -> f32 {
        self.into_normalized_f32_between(T::min_value(), T::max_value(), false)
    }

    fn from_normalized_f32_between(value: f32, min: Self, max: Self, clamp: bool) -> Self {
        debug_assert!(min <= T::zero());
        debug_assert!(max > T::zero());

        let value = if clamp { value.clamp(-1.0, 1.0) } else { value };
        if value >= 0.0 {
            T::from((value * max.to_f32().unwrap()).floor()).unwrap()
        } else {
            T::from((-value * min.to_f32().unwrap()).floor()).unwrap()
        }
    }

    fn from_normalized_f32(value: f32) -> Self {
        T::from_normalized_f32_between(value, T::min_value(), T::max_value(), false)
    }
}
