use std::{fmt::Debug, slice::Iter};

use itertools::Itertools;

pub trait IntoFromF32 {
    fn into_f32(self) -> f32;
    fn from_f32(value: f32) -> Self;
}
macro_rules! impl_into_from_f32 {
    ($type: ty) => {
        impl IntoFromF32 for $type {
            fn into_f32(self) -> f32 {
                self as f32
            }

            fn from_f32(value: f32) -> Self {
                value as Self
            }
        }
    };
}
macro_rules! impl_into_from_f32_round {
    ($type: ty) => {
        impl IntoFromF32 for $type {
            fn into_f32(self) -> f32 {
                self as f32
            }

            fn from_f32(value: f32) -> Self {
                value.round() as Self
            }
        }
    };
}

impl_into_from_f32!(f32);
impl_into_from_f32_round!(i16);
impl_into_from_f32_round!(i32);

pub trait SamplePoint: Copy + Clone + IntoFromF32 + Debug {}

impl SamplePoint for i16 {}
impl SamplePoint for i32 {}
impl SamplePoint for f32 {}

pub enum Interpolation {
    Nearest,
    Linear,
}

pub struct Sample<S: SamplePoint, const CHANNELS: usize> {
    data: [Vec<S>; CHANNELS],
    sample_rate: usize,
}

impl<S: SamplePoint, const CHANNELS: usize> Sample<S, CHANNELS> {
    fn index(&self, index: usize) -> [S; CHANNELS] {
        self.data
            .iter()
            .map(|channel| channel[index])
            .collect_vec()
            .try_into()
            .unwrap()
    }

    fn at_time(&self, time: f32) -> [S; CHANNELS] {
        self.index((time * self.sample_rate as f32) as usize)
    }

    pub fn resample(&self, sample_rate: usize, interpolation: Interpolation) -> Self {
        let mut sample = self.stretch(
            sample_rate as f32 / self.sample_rate as f32,
            Some([S::from_f32(0.0); CHANNELS]),
            interpolation,
        );
        sample.sample_rate = sample_rate;

        sample
    }

    pub fn volume(&self, volume: f32) -> Self {
        let data = self
            .data
            .iter()
            .map(|channel| {
                channel
                    .into_iter()
                    .map(|&sample| (sample).into_f32() * volume)
                    .map(|sample| S::from_f32(sample))
                    .collect_vec()
            })
            .collect_vec();

        Sample {
            data: data.try_into().unwrap(),
            ..*self
        }
    }

    pub fn stretch(
        &self,
        factor: f32,
        last_sample: Option<[S; CHANNELS]>,
        interpolation: Interpolation,
    ) -> Self {
        let len = (self.data[0].len() as f32 * factor).round() as usize;

        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(i_channel, channel)| {
                (0..len)
                    .map(|i_sample| match interpolation {
                        Interpolation::Nearest => {
                            channel[(i_sample as f32 / factor).floor() as usize]
                        }
                        Interpolation::Linear => {
                            let frac = i_sample as f32 / factor;
                            let index = frac.floor() as usize;
                            let frac = frac - index as f32;

                            let sample_1 = channel[index].into_f32();
                            let sample_2 = if channel.len() > index + 1 {
                                channel[index + 1]
                            } else {
                                last_sample.map(|s| s[i_channel]).unwrap_or(channel[index])
                            }
                            .into_f32();

                            S::from_f32((1.0 - frac) * sample_1 + frac * sample_2)
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self {
            data: data.try_into().unwrap(),
            ..*self
        }
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
        let data = [self.data[0].clone(), self.data[0].clone()];

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
        let data = Iter::zip(self.data[0].iter(), self.data[1].iter())
            .map(|(sample1, sample2)| {
                let sample1 = sample1.into_f32();
                let sample2 = sample2.into_f32();

                sample1 * 0.5 + sample2 * 0.5
            })
            .map(|sample| S::from_f32(sample))
            .collect_vec();

        Sample::<S, 1> {
            data: [data].try_into().unwrap(),
            sample_rate: self.sample_rate,
        }
    }
}
