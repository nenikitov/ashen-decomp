use crate::{
    asset::{pack_info::PackInfo, AssetChunk},
    utils::nom::*,
};

#[derive(Debug)]
pub struct TSong {
    bpm: u8,
    speed: u8,
    restart_order: u8,
    orders: Vec<u8>,
    patterns: Vec<TPattern>,
    // TODO(nenikitov) for debug - remove this
    bytes: Vec<u8>,
}

impl TSong {
    // TODO(nenikitov) for debug - remove this
    pub fn debug(&self) {
        std::fs::write("/home/nenikitov/Shared/Documents/Projects/Programming/Rust/ashen-unpacker/output/songs/game/parsed.dat", &self.bytes).unwrap();
        dbg!(&self.patterns);
    }
}

impl AssetChunk for TSong {
    fn parse(input: &[u8]) -> Result<Self> {
        let (header, pointers) = {
            let (input, header) = TSongHeader::parse(input)?;
            let (input, pointers) = TSongPointers::parse(input)?;
            (header, pointers)
        };

        let (_, orders) = multi::count!(number::le_u8, header.song_length as usize)(
            &input[pointers.orders as usize..],
        )?;

        let (_, patterns) = multi::count!(number::le_u32, header.pattern_count as usize)(
            &input[pointers.patterns as usize..],
        )?;
        let patterns: Vec<_> = patterns
            .into_iter()
            .map(|p| p + pointers.pattern_data)
            .map(|p| -> Result<_> { TPattern::parse(&input[p as usize..]) })
            .map(|p| p.and_then(|p| Ok(p.1)))
            .collect::<std::result::Result<_, _>>()?;

        dbg!(&patterns);

        Ok((
            input,
            Self {
                bpm: header.bpm,
                speed: header.speed,
                restart_order: header.restart_order,
                orders,
                patterns,
                // TODO(nenikitov) for debug - remove this
                bytes: input.to_vec(),
            },
        ))
    }
}

#[derive(Debug)]
struct TSongHeader {
    song_length: u8,
    restart_order: u8,
    channel_count: u8,
    pattern_count: u8,
    instrument_count: u8,
    sample_count: u8,
    speed: u8,
    bpm: u8,
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

#[derive(Debug)]
struct TSongPointers {
    orders: u32,
    patterns: u32,
    pattern_lengths: u32,
    pattern_data: u32,
    instruments: u32,
    samples: u32,
    sample_data: u32,
}

impl AssetChunk for TSongPointers {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, orders) = number::le_u32(input)?;
        let (input, patterns) = number::le_u32(input)?;
        let (input, pattern_lengths) = number::le_u32(input)?;
        let (input, pattern_data) = number::le_u32(input)?;
        let (input, instruments) = number::le_u32(input)?;
        let (input, samples) = number::le_u32(input)?;
        let (input, sample_data) = number::le_u32(input)?;

        Ok((
            input,
            Self {
                orders,
                patterns,
                pattern_lengths,
                pattern_data,
                instruments,
                samples,
                sample_data,
            },
        ))
    }
}

#[derive(Debug)]
struct TPattern {
    flags: u8,
    note: u8,
    instrument: u8,
    volume: u8,
    effect_1: u16,
    effect_2: u16,
}

impl AssetChunk for TPattern {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, flags) = number::le_u8(input)?;
        let (input, note) = number::le_u8(input)?;
        let (input, instrument) = number::le_u8(input)?;
        let (input, volume) = number::le_u8(input)?;
        let (input, effect_1) = number::le_u16(input)?;
        let (input, effect_2) = number::le_u16(input)?;

        Ok((
            input,
            Self {
                flags,
                note,
                instrument,
                volume,
                effect_1,
                effect_2,
            },
        ))
    }
}
