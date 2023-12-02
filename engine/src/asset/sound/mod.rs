mod dat;

use flate2::read::ZlibDecoder;
use std::{io::Read, ops::Index};

use super::{pack_info::PackInfo, Asset, AssetChunk, Extension, Kind};
use crate::{
    asset::sound::dat::{
        asset_header::SongAssetHeader, chunk_header::SongChunkHeader, t_song::TSong,
    },
    error::{self, ParseError},
    utils::nom::*,
};

pub struct SoundAssetCollection {}

impl SoundAssetCollection {
    // TODO(nenikitov): Move to utils
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
}

impl Asset for SoundAssetCollection {
    fn kind() -> Kind {
        Kind::SoundCollection
    }

    fn parse(input: &[u8], extension: Extension) -> Result<Self> {
        match extension {
            Extension::Dat => {
                let (_, header) = SongAssetHeader::parse(input)?;

                let songs = {
                    let (_, songs) = SongChunkHeader::parse(&input[header.songs])?;

                    songs
                        .songs
                        .into_iter()
                        .map(|s| Self::deflate(&input[s]))
                        // TODO(nenikitov): use `Result` properly
                        .map(|s| TSong::parse(s.as_slice()).unwrap().1)
                        .collect::<Vec<_>>()
                };

                std::fs::write(
                    format!("/home/nenikitov/Shared/Documents/Projects/Programming/Rust/ashen-unpacker/output/songs/game/out.pcm"),
                    songs[0xc].mix().iter().flat_map(|d| d.to_le_bytes()).collect::<Vec<u8>>()
                );

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
