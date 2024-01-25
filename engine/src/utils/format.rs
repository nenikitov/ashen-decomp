use crate::asset::color_map::Color;
use image::{
    codecs::{
        gif::{GifEncoder, Repeat},
        png::PngEncoder,
    },
    Frame, ImageEncoder, RgbaImage,
};
use std::ops::Deref;

pub trait PngFile {
    fn to_png(&self) -> Vec<u8>;
}

// impl for any 2D array like data structure.
impl<Outer: ?Sized, Inner> PngFile for Outer
where
    Outer: Deref<Target = [Inner]>,
    Inner: AsRef<[Color]>,
{
    fn to_png(&self) -> Vec<u8> {
        let width = self[0].as_ref().len() as u32;
        let height = self.len() as u32;

        let mut data = vec![];
        {
            let mut encoder = PngEncoder::new(&mut data);

            encoder.write_image(
                &self
                    .iter()
                    .flat_map(|slice| {
                        slice
                            .as_ref()
                            .iter()
                            .flat_map(|color| [color.r, color.g, color.b])
                    })
                    .collect::<Vec<_>>(),
                width,
                height,
                image::ColorType::Rgb8,
            );
        }

        data
    }
}

pub trait GifFile {
    fn to_gif(&self) -> Vec<u8>;
}

impl<Outer: ?Sized, Inner1, Inner2> GifFile for Outer
where
    Outer: Deref<Target = [Inner1]>,
    Inner1: Deref<Target = [Inner2]>,
    Inner2: AsRef<[Color]>,
{
    fn to_gif(&self) -> Vec<u8> {
        let width = self[0][0].as_ref().len() as u32;
        let height = self[0].len() as u32;

        let mut data = vec![];
        {
            let mut encoder = GifEncoder::new_with_speed(&mut data, 10);

            encoder
                .encode_frames(self.iter().map(|f| {
                    Frame::new(
                        RgbaImage::from_vec(
                            width,
                            height,
                            f.iter()
                                .flat_map(|slice| {
                                    slice
                                        .as_ref()
                                        .iter()
                                        .flat_map(|color| [color.r, color.g, color.b, 255])
                                })
                                .collect(),
                        )
                        .expect("Generated image data must be valid"),
                    )
                }))
                .expect("Generated image frames must be valid");

            encoder.set_repeat(Repeat::Infinite);
        }
        data
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
