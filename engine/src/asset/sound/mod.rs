mod dat;

use flate2::read::ZlibDecoder;
use std::{
    fs,
    io::{self, Read},
    ops::Index,
    path::Path,
};

use self::dat::mixer::SoundEffect;

use super::{pack_info::PackInfo, Asset, AssetChunk, Extension, Kind};
use crate::{
    asset::sound::dat::{
        asset_header::SongAssetHeader, chunk_header::SongChunkHeader, t_song::TSong,
    },
    error::{self, ParseError},
    utils::nom::*,
};

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

pub struct SoundAssetCollection {
    songs: Vec<TSong>,
}

impl SoundAssetCollection {
    fn save<P>(&self, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let dir = path.as_ref();
        fs::create_dir_all(dir)?;

        for (i, song) in self.songs.iter().enumerate() {
            fs::write(dir.join(format!("{i:X}.wav")), song.mix().to_wave())?
        }

        Ok(())
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

                let (_, songs) = SongChunkHeader::parse(&input[header.songs])?;

                let songs = songs
                    .songs
                    .into_iter()
                    .map(|s| deflate(&input[s]))
                    .map(|s| TSong::parse(s.as_slice()).map(|(_, d)| d))
                    .collect::<std::result::Result<Vec<_>, _>>()?;

                Ok((&[], SoundAssetCollection { songs }))
            }
            _ => Err(error::ParseError::unsupported_extension(input, extension).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_songs() -> eyre::Result<()> {
        let bytes = fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../output/deflated/BBC974.dat"
        ))?;

        let (_, sac) = SoundAssetCollection::parse(&bytes, Extension::Dat)?;

        sac.save(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../output/songs/game/"
        ))?;

        Ok(())
    }
}
