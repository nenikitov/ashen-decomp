use flate2::read::ZlibDecoder;
use std::io::Read;

use super::{Asset, Extension, Kind};
use crate::{
    error::{self, ParseError},
    utils::nom::*,
};

pub struct SoundAssetCollection {}

#[derive(Debug)]
struct PackInfo {
    offset: u32,
    size: u32,
}

#[derive(Debug)]
struct Song {
    song_length: u8,
    restart_order: u8,
    channel_count: u8,
    pattern_count: u8,
    instrument_count: u8,
    speed: u8,
    bpm: u8,
}

impl SoundAssetCollection {
    const HEADER: &'static str = "TSND";

    fn pack_info(input: &[u8]) -> Result<PackInfo> {
        let (input, offset) = number::le_u32(input)?;
        let (input, size) = number::le_u32(input)?;

        Ok((input, PackInfo { offset, size }))
    }

    fn song_chunk_header(input: &[u8]) -> Result<Vec<PackInfo>> {
        let (input, song_count) = number::le_u32(input)?;
        // TODO(nenikitov): Figure out how the `song_count` works
        let (input, offsets) = multi::count!(Self::pack_info, 1)(input)?;

        Ok((input, offsets))
    }

    // TODO(nenikitov): Return `Result`
    fn deflate(input: &[u8]) -> Vec<u8> {
        if let [b'Z', b'L', s1, s2, s3, bytes @ ..] = input {
            let size = u32::from_le_bytes([*s1, *s2, *s3, 0]);

            let mut decoder = ZlibDecoder::new(bytes);
            let mut data = Vec::with_capacity(size as usize);
            decoder
                .read_to_end(&mut data)
                .expect("Data should be a valid zlib stream");

            data
        } else {
            input.to_vec()
        }
    }

    fn song(input: &[u8]) -> Result<Song> {
        let (input, song_length) = number::le_u8(input)?;
        let (input, restart_order) = number::le_u8(input)?;
        let (input, channel_count) = number::le_u8(input)?;
        let (input, pattern_count) = number::le_u8(input)?;
        let (input, instrument_count) = number::le_u8(input)?;
        let (input, speed) = number::le_u8(input)?;
        let (input, bpm) = number::le_u8(input)?;

        let song = Song {
            song_length,
            restart_order,
            channel_count,
            pattern_count,
            instrument_count,
            speed,
            bpm,
        };

        dbg!(&song);

        Ok((input, song))
    }
}

impl Asset for SoundAssetCollection {
    fn kind() -> Kind {
        Kind::SoundCollection
    }

    fn parse(input: &[u8], extension: Extension) -> Result<Self> {
        match extension {
            Extension::Dat => {
                let (songs, effects, emitters, maps) = {
                    let (input, _) = bytes::tag(Self::HEADER)(input)?;

                    let (input, songs) = Self::pack_info(input)?;
                    let (input, effects) = Self::pack_info(input)?;
                    let (input, emitters) = Self::pack_info(input)?;
                    let (input, maps) = Self::pack_info(input)?;

                    (songs, effects, emitters, maps)
                };

                let songs = {
                    let (_, songs) = Self::song_chunk_header(
                        &input[songs.offset as usize..][..songs.size as usize],
                    )?;

                    songs
                        .iter()
                        .map(|s| -> std::result::Result<Song, ParseError> {
                            // TODO(nenikitov): use `Result` properly
                            let deflated =
                                Self::deflate(&input[s.offset as usize..][..s.size as usize]);
                            let (_, s) = Self::song(deflated.as_slice()).unwrap();
                            Ok(s)
                        })
                        .collect::<std::result::Result<Vec<_>, _>>()?;
                };

                todo!()
            }
            _ => Err(error::ParseError::unsupported_extension(input, extension).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // TODO(nenikitov): Remove this
        SoundAssetCollection::parse(
            include_bytes!("../../../../output/deflated/BBC974.dat"),
            Extension::Dat,
        );
    }
}
