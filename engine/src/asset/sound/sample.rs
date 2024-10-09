// TODO(nenikitov): Remove this test code
use std::{fmt::Debug, mem::size_of, ops::Index};

pub enum AudioSamplePointFormat {
    Int,
    Float,
}

impl AudioSamplePointFormat {
    pub const fn signature(&self) -> u16 {
        match self {
            AudioSamplePointFormat::Int => 1,
            AudioSamplePointFormat::Float => 3,
        }
    }
}

pub trait AudioSamplePoint: Default + Clone + Copy {
    const SIZE_BYTES: usize = size_of::<Self>();

    fn into_normalized_f32(&self) -> f32;
    fn from_normalized_f32(value: f32) -> Self;

    fn wave_format() -> AudioSamplePointFormat;
    fn wave_le_bytes(&self) -> [u8; Self::SIZE_BYTES];
}

impl AudioSamplePoint for i16 {
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

    fn wave_format() -> AudioSamplePointFormat {
        AudioSamplePointFormat::Int
    }

    fn wave_le_bytes(&self) -> [u8; Self::SIZE_BYTES] {
        self.to_le_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct AudioBuffer<S: AudioSamplePoint> {
    pub data: Vec<S>,
    pub sample_rate: usize,
}

impl<S: AudioSamplePoint> AudioBuffer<S> {
    pub fn index_to_seconds(&self, index: usize) -> f64 {
        index as f64 / self.sample_rate as f64
    }

    pub fn seconds_to_index(&self, seconds: f64) -> usize {
        (seconds * self.sample_rate as f64) as usize
    }

    pub fn len_samples(&self) -> usize {
        self.data.len()
    }

    pub fn len_seconds(&self) -> f64 {
        self.index_to_seconds(self.len_samples())
    }
}

impl<S: AudioSamplePoint> Index<usize> for AudioBuffer<S> {
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}
