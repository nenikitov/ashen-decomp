use std::fmt::Debug;

use itertools::Itertools;

pub trait SamplePointConversions {
    const BITS: usize;

    fn into_f32(self) -> f32;
    fn from_f32(value: f32) -> Self;
    fn to_le_bytes(self) -> Vec<u8>;
}

macro_rules! impl_sample_point_conversions_other {
    ($type: ty, $size: literal) => {
        impl SamplePointConversions for $type {
            const BITS: usize = $size as usize;

            fn into_f32(self) -> f32 {
                self as f32
            }

            fn from_f32(value: f32) -> Self {
                value as Self
            }

            fn to_le_bytes(self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        }
    };
}
macro_rules! impl_sample_point_conversions_integer {
    ($type: tt) => {
        impl SamplePointConversions for $type {
            const BITS: usize = $type::BITS as usize;

            fn into_f32(self) -> f32 {
                self as f32
            }

            fn from_f32(value: f32) -> Self {
                value.round() as Self
            }

            fn to_le_bytes(self) -> Vec<u8> {
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
pub enum Interpolation {
    Nearest,
    Linear,
}

pub struct Sample<S: SamplePoint, const CHANNELS: usize> {
    pub data: [Vec<S>; CHANNELS],
    pub sample_rate: usize,
}

impl<S: SamplePoint, const CHANNELS: usize> Sample<S, CHANNELS> {
    pub fn index(&self, index: usize) -> [S; CHANNELS] {
        self.data
            .iter()
            .map(|channel| channel[index])
            .collect_vec()
            .try_into()
            .unwrap()
    }

    pub fn len(&self) -> usize {
        self.data[0].len()
    }

    pub fn at_time(&self, time: f32) -> [S; CHANNELS] {
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

    pub fn add_sample(&mut self, other: Sample<S, CHANNELS>, offset: usize) {
        let new_len = offset + other.len();

        if new_len > self.len() {
            for channel in self.data.iter_mut() {
                channel.resize(new_len, S::from_f32(0.0));
            }
        }

        for (channel_self, channel_other) in Iterator::zip(self.data.iter_mut(), other.data.iter())
        {
            for (i, s) in channel_other.iter().enumerate() {
                let i = i + offset;
                channel_self[i] = S::from_f32(channel_self[i].into_f32() + s.into_f32());
            }
        }
    }

    pub fn volume(&self, volume: f32) -> Self {
        let data = self
            .data
            .iter()
            .map(|channel| {
                channel
                    .iter()
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
                                last_sample.map_or(channel[index], |s| s[i_channel])
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
        let data = Iterator::zip(self.data[0].iter(), self.data[1].iter())
            .map(|(sample1, sample2)| {
                let sample1 = sample1.into_f32();
                let sample2 = sample2.into_f32();

                sample1 * 0.5 + sample2 * 0.5
            })
            .map(|sample| S::from_f32(sample))
            .collect_vec();

        Sample::<S, 1> {
            data: [data],
            sample_rate: self.sample_rate,
        }
    }
}
