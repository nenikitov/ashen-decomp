use std::io;

use lewton::inside_ogg::OggStreamReader;

use crate::{
    asset::{sound::dat::t_instrument::TSampleParsed, AssetChunk},
    utils::nom::*,
};

use super::{
    mixer::Mixer,
    t_instrument::{TInstrument, TSample},
};

pub struct TEffect {
    instrument: TInstrument,
    sample: TSampleParsed,
}

// It should be separated
impl TEffect {
    // TODO(nenikitov): This is duplicated code from `TSong`

    fn uncompress(bytes: &[u8]) -> Vec<i16> {
        if let [b'V', b'B', u1, u2, u3, c1, c2, c3, bytes @ ..] = bytes {
            let size_uncompressed = u32::from_le_bytes([*u1, *u2, *u3, 0]);
            let size_compressed = u32::from_le_bytes([*c1, *c2, *c3, 0]);

            let bytes = &bytes[..size_compressed as usize];

            let mut data = OggStreamReader::new(io::Cursor::new(bytes))
                .expect("Sample should be a valid OGG stream");
            let mut samples: Vec<_> = Vec::with_capacity(size_uncompressed as usize / 2);
            while let Ok(Some(packet)) = data.read_dec_packet_itl() {
                samples.extend(packet);
            }

            samples
        } else {
            bytes
                .array_chunks::<1>()
                .map(|bytes| i8::from_le_bytes(*bytes) as i16)
                .map(|sample| sample * 256)
                .collect()
        }
    }

    pub fn mix(&self) -> Vec<i16> {
        let mut m = Mixer::new();

        m.add_samples(&self.sample.sample, 0);

        m.mix()
    }
}

impl AssetChunk for TEffect {
    fn parse(input: &[u8]) -> crate::utils::nom::Result<Self> {
        let (_, pointers) = TEffectPointers::parse(input)?;

        let (_, instrument) = TInstrument::parse(&input[pointers.instrument as usize..])?;

        let sample = {
            let data = Self::uncompress(&input[pointers.sample_data as usize..]);
            let (_, sample) = TSample::parse(&input[pointers.sample as usize..])?;

            TSampleParsed::parse(
                &sample,
                &data[sample.sample as usize..sample.loop_end as usize],
            )
        };

        Ok((&[], Self { instrument, sample }))
    }
}

#[derive(Debug)]
struct TEffectPointers {
    instrument: u32,
    sample: u32,
    sample_data: u32,
}

impl AssetChunk for TEffectPointers {
    fn parse(input: &[u8]) -> crate::utils::nom::Result<Self> {
        let (input, instrument) = number::le_u32(input)?;
        let (input, sample) = number::le_u32(input)?;
        let (input, sample_data) = number::le_u32(input)?;

        Ok((
            input,
            Self {
                instrument,
                sample,
                sample_data,
            },
        ))
    }
}
