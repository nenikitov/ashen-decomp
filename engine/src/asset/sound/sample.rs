// TODO(nenikitov): Remove this test code
use std::{fmt::Debug, ops::Index};

pub trait SamplePoint: Default + Clone + Copy {
    fn into_normalized_f32(&self) -> f32;
    fn from_normalized_f32(value: f32) -> Self;
}

macro_rules! impl_sample_point_for {
    ($($ty:ty),*) => {
        $(impl SamplePoint for $ty {
            fn into_normalized_f32(&self) -> f32 {
                if *self < 0 {
                    -(*self as f32 / Self::MIN as f32)
                } else {
                    (*self as f32 / Self::MAX as f32)
                }
            }

            fn from_normalized_f32(value: f32) -> Self {
                if value < 0. {
                    -(value * Self::MIN as f32) as Self
                } else {
                    (value * Self::MAX as f32) as Self
                }
            }
        })*
    }
}

impl_sample_point_for!(i16);

#[derive(Debug, Clone)]
pub struct AudioBuffer<S: SamplePoint> {
    pub data: Vec<S>,
    pub sample_rate: usize,
}

impl<S: SamplePoint> AudioBuffer<S> {
    pub fn index_to_seconds(&self, index: usize) -> f64 {
        index as f64 / self.sample_rate as f64
    }

    pub fn seconds_to_index(&self, seconds: f64) -> usize {
        (seconds * self.sample_rate as f64) as usize
    }

    pub fn len_seconds(&self) -> f64 {
        self.index_to_seconds(self.data.len())
    }
}

impl<S: SamplePoint> Index<usize> for AudioBuffer<S> {
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}
