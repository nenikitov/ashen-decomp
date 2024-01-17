use crate::asset::color_map::Color;
use std::ops::Deref;

pub trait PpmFile {
    fn to_ppm(&self) -> Vec<u8>;
}

// impl for any 2D array like data structure.
impl<Outer: ?Sized, Inner> PpmFile for Outer
where
    Outer: Deref<Target = [Inner]>,
    Inner: AsRef<[Color]>,
{
    fn to_ppm(&self) -> Vec<u8> {
        let width = self[0].as_ref().len();
        let height = self.len();

        let mut bytes = format!("P6 {width} {height} 255\n").into_bytes();
        bytes.extend(self.iter().flat_map(|slice| {
            slice
                .as_ref()
                .iter()
                .flat_map(|color| [color.r, color.g, color.b])
        }));

        bytes
    }
}

pub trait WaveFile {
    fn to_wave(&self, sample_rate: usize, channel_count: usize) -> Vec<u8>;
}

impl WaveFile for Vec<i16> {
    fn to_wave(&self, sample_rate: usize, channel_count: usize) -> Vec<u8> {
        const BITS_PER_SAMPLE: usize = 16;
        const BYTES_PER_SAMPLE: usize = BITS_PER_SAMPLE / 8;

        let size = self.len() * BYTES_PER_SAMPLE;

        "RIFF"
            .bytes()
            .chain(u32::to_be_bytes((36 + size) as u32))
            .chain("WAVE".bytes())
            .chain("fmt ".bytes())
            .chain(u32::to_le_bytes(16))
            .chain(u16::to_le_bytes(1))
            .chain(u16::to_le_bytes(channel_count as u16))
            .chain(u32::to_le_bytes(sample_rate as u32))
            .chain(u32::to_le_bytes(
                (sample_rate * channel_count * BYTES_PER_SAMPLE) as u32,
            ))
            .chain(u16::to_le_bytes((channel_count * BYTES_PER_SAMPLE) as u16))
            .chain(u16::to_le_bytes(BITS_PER_SAMPLE as u16))
            .chain("data".bytes())
            .chain(u32::to_le_bytes(size as u32))
            .chain(self.iter().flat_map(|s| s.to_le_bytes()))
            .collect()
    }
}
