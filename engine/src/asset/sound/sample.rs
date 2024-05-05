// TODO(nenikitov): Remove this test code
use std::{
    fmt::Debug,
    ops::{Index, Range, RangeFrom, RangeTo},
};

use itertools::Itertools;

use crate::utils::iterator::CollectArray;

pub trait SamplePointConversions
where
    Self: Sized,
{
    const SIZE_BITS: usize;

    fn into_f32(self) -> f32;
    fn from_f32(value: f32) -> Self;

    fn to_integer_le_bytes(self) -> Vec<u8>;
}

macro_rules! impl_sample_point_conversions_other {
    ($type: ty, $size: literal) => {
        impl SamplePointConversions for $type {
            const SIZE_BITS: usize = $size as usize;

            fn into_f32(self) -> f32 {
                self as f32
            }

            fn from_f32(value: f32) -> Self {
                value as Self
            }

            fn to_integer_le_bytes(self) -> Vec<u8> {
                ((self.clamp(-1.0, 1.0) * i32::MAX as f32) as i32)
                    .to_le_bytes()
                    .to_vec()
            }
        }
    };
}
macro_rules! impl_sample_point_conversions_integer {
    ($type: tt) => {
        impl SamplePointConversions for $type {
            const SIZE_BITS: usize = $type::BITS as usize;

            fn into_f32(self) -> f32 {
                self as f32 / $type::MAX as f32
            }

            fn from_f32(value: f32) -> Self {
                (value * $type::MAX as f32) as Self
            }

            fn to_integer_le_bytes(self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        }
    };
}

impl_sample_point_conversions_other!(f32, 32);
impl_sample_point_conversions_integer!(i16);
impl_sample_point_conversions_integer!(i32);

pub trait SamplePoint: Copy + Clone + SamplePointConversions + Debug {}

impl SamplePoint for i16 {}
impl SamplePoint for i32 {}
impl SamplePoint for f32 {}

#[derive(Debug, Clone, Copy)]
pub enum Interpolation<S: SamplePoint, const CHANNELS: usize> {
    Nearest,
    Linear {
        first_sample_after: Option<[S; CHANNELS]>,
    },
}

#[derive(Debug, Clone)]
pub struct Sample<S: SamplePoint, const CHANNELS: usize> {
    pub data: Vec<[S; CHANNELS]>,
    pub sample_rate: usize,
}

impl<S: SamplePoint, const CHANNELS: usize> Index<usize> for Sample<S, CHANNELS> {
    type Output = [S; CHANNELS];

    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<S: SamplePoint, const CHANNELS: usize> Index<Range<usize>> for Sample<S, CHANNELS> {
    type Output = [[S; CHANNELS]];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.data[range]
    }
}

impl<S: SamplePoint, const CHANNELS: usize> Index<RangeFrom<usize>> for Sample<S, CHANNELS> {
    type Output = [[S; CHANNELS]];

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.data[range]
    }
}

impl<S: SamplePoint, const CHANNELS: usize> Index<RangeTo<usize>> for Sample<S, CHANNELS> {
    type Output = [[S; CHANNELS]];

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.data[range]
    }
}

impl<S: SamplePoint, const CHANNELS: usize> Index<f32> for Sample<S, CHANNELS> {
    type Output = [S; CHANNELS];

    fn index(&self, index: f32) -> &Self::Output {
        self.data.index(self.time_to_index(index))
    }
}

impl<S: SamplePoint, const CHANNELS: usize> Index<Range<f32>> for Sample<S, CHANNELS> {
    type Output = [[S; CHANNELS]];

    fn index(&self, range: Range<f32>) -> &Self::Output {
        &self.data[self.time_to_index(range.start)..self.time_to_index(range.end)]
    }
}

impl<S: SamplePoint, const CHANNELS: usize> Index<RangeFrom<f32>> for Sample<S, CHANNELS> {
    type Output = [[S; CHANNELS]];

    fn index(&self, range: RangeFrom<f32>) -> &Self::Output {
        &self.data[self.time_to_index(range.start)..]
    }
}

impl<S: SamplePoint, const CHANNELS: usize> Index<RangeTo<f32>> for Sample<S, CHANNELS> {
    type Output = [[S; CHANNELS]];

    fn index(&self, range: RangeTo<f32>) -> &Self::Output {
        &self.data[..self.time_to_index(range.end)]
    }
}

impl<S: SamplePoint, const CHANNELS: usize> Sample<S, CHANNELS> {
    fn time_to_index(&self, time: f32) -> usize {
        (time * self.sample_rate as f32) as usize
    }

    pub fn len_samples(&self) -> usize {
        self.data.len()
    }

    pub fn len_seconds(&self) -> f32 {
        self.data.len() as f32 / self.sample_rate as f32
    }

    pub fn resample(&self, sample_rate: usize, interpolation: Interpolation<S, CHANNELS>) -> Self {
        let data = self
            .data
            .stretch(sample_rate as f32 / self.sample_rate as f32, interpolation);

        Self {
            data,
            sample_rate: self.sample_rate,
        }
    }
}

pub trait SamplePointProcessing<S: SamplePoint, const CHANNELS: usize> {
    fn add_sample(&mut self, other: &[S; CHANNELS]);
    fn volume(&self, volume: f32) -> Self;
}

impl<S: SamplePoint, const CHANNELS: usize> SamplePointProcessing<S, CHANNELS> for [S; CHANNELS] {
    fn add_sample(&mut self, other: &[S; CHANNELS]) {
        for channel_i in 0..CHANNELS {
            self[channel_i] = S::from_f32(self[channel_i].into_f32() + other[channel_i].into_f32());
        }
    }

    fn volume(&self, volume: f32) -> Self {
        self.iter()
            .map(|sample| sample.into_f32() * volume)
            .map(S::from_f32)
            .collect_array::<CHANNELS>()
    }
}

pub trait SampleDataProcessing<S: SamplePoint, const CHANNELS: usize> {
    fn add_sample(&mut self, other: &[[S; CHANNELS]], offset: usize);
    fn volume(&self, volume: f32) -> Self;
    fn stretch(&self, factor: f32, interpolation: Interpolation<S, CHANNELS>) -> Self;
}

impl<S: SamplePoint, const CHANNELS: usize> SampleDataProcessing<S, CHANNELS>
    for Vec<[S; CHANNELS]>
{
    fn add_sample(&mut self, other: &[[S; CHANNELS]], offset: usize) {
        let new_len = offset + other.len();

        if new_len > self.len() {
            self.resize(new_len, [S::from_f32(0.0); CHANNELS]);
        }

        for (i, samples_other) in other.iter().enumerate() {
            let i = i + offset;

            for channel_i in 0..CHANNELS {
                self[i].add_sample(samples_other);
            }
        }
    }

    fn volume(&self, volume: f32) -> Self {
        self.iter().map(|samples| samples.volume(volume)).collect()
    }

    fn stretch(&self, factor: f32, interpolation: Interpolation<S, CHANNELS>) -> Self {
        let len = (self.len() as f32 * factor).round() as usize;

        (0..len)
            .map(|(i_sample)| {
                (0..CHANNELS)
                    .map(|i_channel| match interpolation {
                        Interpolation::Nearest => {
                            self[(i_sample as f32 / factor).floor() as usize][i_channel]
                        }
                        Interpolation::Linear {
                            first_sample_after: last_sample,
                        } => {
                            let frac = i_sample as f32 / factor;
                            let index = frac.floor() as usize;
                            let frac = frac - index as f32;

                            let sample_1 = self[index][i_channel].into_f32();
                            let sample_2 = if self.len() > index + 1 {
                                self[index + 1][i_channel]
                            } else {
                                last_sample.map_or(self[index][i_channel], |s| s[i_channel])
                            }
                            .into_f32();

                            S::from_f32((1.0 - frac) * sample_1 + frac * sample_2)
                        }
                    })
                    .collect_array::<CHANNELS>()
            })
            .collect()
    }
}

impl<S: SamplePoint> Sample<S, 1> {
    pub fn mono(sample_rate: usize) -> Self {
        Self {
            data: Default::default(),
            sample_rate,
        }
    }

    pub fn to_stereo(&self) -> Sample<S, 2> {
        let data = self.data.iter().map(|[s]| [*s, *s]).collect();

        Sample::<S, 2> {
            data,
            sample_rate: self.sample_rate,
        }
    }
}

impl<S: SamplePoint> Sample<S, 2> {
    pub fn stereo(sample_rate: usize) -> Self {
        Self {
            data: Default::default(),
            sample_rate,
        }
    }

    pub fn to_mono(&self) -> Sample<S, 1> {
        let data = self
            .data
            .iter()
            .map(|[sample_1, sample_2]| {
                let sample_1 = sample_1.into_f32();
                let sample_2 = sample_2.into_f32();

                [S::from_f32(sample_1 * 0.5 + sample_2 * 0.5)]
            })
            .collect_vec();

        Sample::<S, 1> {
            data,
            sample_rate: self.sample_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume() {
        let samples = vec![
            [11, 20],
            [-20, -40],
            [30, 60],
            [-40, -80],
            [-50, -100],
            [-80, -160],
        ];

        assert_eq!(
            samples.volume(1.47),
            vec![
                [16, 29],
                [-29, -59],
                [44, 88],
                [-59, -118],
                [-74, -147],
                [-118, -235],
            ]
        );

        assert_eq!(
            samples.volume(0.5),
            vec![
                [6, 10],
                [-10, -20],
                [15, 30],
                [-20, -40],
                [-25, -50],
                [-40, -80],
            ]
        );
    }

    #[test]
    fn stretch_nearest_integer() {
        let samples = vec![
            [10, 20],
            [-20, -40],
            [30, 60],
            [-40, -80],
            [-50, -100],
            [-80, -160],
        ];

        assert_eq!(
            samples.stretch(2.0, Interpolation::Nearest),
            vec![
                [10, 20],
                [10, 20],
                [-20, -40],
                [-20, -40],
                [30, 60],
                [30, 60],
                [-40, -80],
                [-40, -80],
                [-50, -100],
                [-50, -100],
                [-80, -160],
                [-80, -160],
            ]
        );
    }

    #[test]
    fn stretch_nearest_frac() {
        let samples = vec![
            [10, 20],
            [-20, -40],
            [30, 60],
            [-40, -80],
            [-50, -100],
            [-80, -160],
        ];

        assert_eq!(
            samples.stretch(1.5, Interpolation::Nearest),
            vec![
                [10, 20],
                [10, 20],
                [-20, -40],
                [30, 60],
                [30, 60],
                [-40, -80],
                [-50, -100],
                [-50, -100],
                [-80, -160],
            ]
        );
    }

    #[test]
    fn stretch_linear_integer_without_last() {
        let samples = vec![
            [10, 20],
            [-20, -40],
            [30, 60],
            [-40, -80],
            [-50, -100],
            [-80, -160],
        ];

        assert_eq!(
            samples.stretch(
                2.0,
                Interpolation::Linear {
                    first_sample_after: None
                }
            ),
            vec![
                [10, 20],
                [-5, -10],
                [-20, -40],
                [5, 10],
                [30, 60],
                [-5, -10],
                [-40, -80],
                [-45, -90],
                [-50, -100],
                [-65, -130],
                [-80, -160],
                [-80, -160],
            ]
        );
    }

    #[test]
    fn stretch_linear_integer_with_last() {
        let samples = vec![
            [10, 20],
            [-20, -40],
            [30, 60],
            [-40, -80],
            [-50, -100],
            [-80, -160],
        ];

        assert_eq!(
            samples.stretch(
                2.0,
                Interpolation::Linear {
                    first_sample_after: Some([500, 500])
                }
            ),
            vec![
                [10, 20],
                [-5, -10],
                [-20, -40],
                [5, 10],
                [30, 60],
                [-5, -10],
                [-40, -80],
                [-45, -90],
                [-50, -100],
                [-65, -130],
                [-80, -160],
                [210, 170],
            ]
        );
    }
}
