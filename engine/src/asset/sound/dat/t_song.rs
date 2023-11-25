use crate::{
    asset::{pack_info::PackInfo, AssetChunk},
    utils::nom::*,
};

#[derive(Debug)]
pub struct TSong {}

impl AssetChunk for TSong {
    fn parse(input: &[u8]) -> Result<Self> {
        // TODO(nenikitov) remove this
        std::fs::write("/home/nenikitov/Shared/Documents/Projects/Programming/Rust/ashen-unpacker/output/songs/main-menu.dat", input).unwrap();

        let (_, header) = TSongHeader::parse(input)?;

        Ok((input, Self {}))
    }
}

#[derive(Debug)]
struct TSongHeader {
    pub song_length: u8,
    pub restart_order: u8,
    pub channel_count: u8,
    pub pattern_count: u8,
    pub instrument_count: u8,
    pub sample_count: u8,
    pub speed: u8,
    pub bpm: u8,
}

impl AssetChunk for TSongHeader {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, song_length) = number::le_u8(input)?;
        let (input, restart_order) = number::le_u8(input)?;
        let (input, channel_count) = number::le_u8(input)?;
        let (input, pattern_count) = number::le_u8(input)?;
        let (input, instrument_count) = number::le_u8(input)?;
        let (input, sample_count) = number::le_u8(input)?;
        let (input, speed) = number::le_u8(input)?;
        let (input, bpm) = number::le_u8(input)?;

        Ok((
            input,
            Self {
                song_length,
                restart_order,
                channel_count,
                pattern_count,
                instrument_count,
                sample_count,
                speed,
                bpm,
            },
        ))
    }
}
