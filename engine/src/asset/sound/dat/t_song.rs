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
    fn uncompress(bytes: &[u8]) -> Vec<u8> {
        if let [b'V', b'B', u1, u2, u3, c1, c2, c3, bytes @ ..] = bytes {
            let size_uncompressed = u32::from_le_bytes([*u1, *u2, *u3, 0]);
            let size_compressed = u32::from_le_bytes([*c1, *c2, *c3, 0]);

            let bytes = &bytes[..size_compressed as usize];

            bytes.to_vec()
        } else {
            bytes.to_vec()
        }
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

        let patterns = multi::count!(number::le_u32, header.pattern_count as usize)(
            &input[pointers.patterns as usize..],
        )?
        .1
        .into_iter()
        .map(|p| p + pointers.pattern_data)
        .map(|p| -> Result<_> { TPattern::parse(&input[p as usize..]) })
        .map(|p| p.and_then(|p| Ok(p.1)))
        .collect::<std::result::Result<_, _>>()?;

        let (_, instruments) = multi::count!(TInstrument::parse, header.instrument_count as usize)(
            &input[pointers.instruments as usize..],
        )?;

        let (_, samples) = multi::count!(TSample::parse, header.sample_count as usize)(
            &input[pointers.samples as usize..],
        )?;

        let sample_data = Self::uncompress(&input[pointers.sample_data as usize..]);

        // TODO(nenikitov): Remove this after done debugging
        std::fs::write("/home/nenikitov/Shared/Documents/Projects/Programming/Rust/ashen-unpacker/output/songs/main-menu-samples.ogg", &sample_data).unwrap();

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

#[derive(Debug)]
struct TInstrument {
    flags: u8,

    volume_begin: u16,
    volume_end: u16,
    volume_sustain: u16,
    volume_envelope_border: u16,
    volume_envelope: Box<[u8; 325]>,

    pan_begin: u16,
    pan_end: u16,
    pan_sustain: u16,
    pan_envelope_border: u16,
    pan_envelope: Box<[u8; 325]>,

    vibrato_depth: u8,
    vibrato_speed: u8,
    vibrato_sweep: u8,

    fadeout: u32,
    vibrato_table: u32,

    samples: Box<[u8; 96]>,
}

impl AssetChunk for TInstrument {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, flags) = number::le_u8(input)?;

        let (input, _) = bytes::take(1usize)(input)?;

        let (input, volume_begin) = number::le_u16(input)?;
        let (input, volume_end) = number::le_u16(input)?;
        let (input, volume_sustain) = number::le_u16(input)?;
        let (input, volume_envelope_border) = number::le_u16(input)?;
        let (input, volume_envelope) = multi::count!(number::le_u8)(input)?;

        let (input, pan_begin) = number::le_u16(input)?;
        let (input, pan_end) = number::le_u16(input)?;
        let (input, pan_sustain) = number::le_u16(input)?;
        let (input, pan_envelope_border) = number::le_u16(input)?;
        // TODO(nenikitov): Figure out why Rust can't figure out this type
        let (input, pan_envelope): (&[u8], [u8; 325]) = multi::count!(number::le_u8)(input)?;

        let (input, _) = bytes::take(1usize)(input)?;

        let (input, vibrato_depth) = number::le_u8(input)?;
        let (input, vibrato_speed) = number::le_u8(input)?;
        let (input, vibrato_sweep) = number::le_u8(input)?;

        let (input, fadeout) = number::le_u32(input)?;
        let (input, vibrato_table) = number::le_u32(input)?;

        let (input, samples) = multi::count!(number::le_u8)(input)?;

        Ok((
            input,
            Self {
                flags,
                volume_begin,
                volume_end,
                volume_sustain,
                volume_envelope_border,
                volume_envelope: Box::new(volume_envelope),
                pan_begin,
                pan_end,
                pan_sustain,
                pan_envelope_border,
                pan_envelope: Box::new(volume_envelope),
                vibrato_depth,
                vibrato_speed,
                vibrato_sweep,
                fadeout,
                vibrato_table,
                samples: Box::new(samples),
            },
        ))
    }
}

#[derive(Debug)]
struct TSample {
    flags: u8,
    volume: u8,
    panning: u8,
    align: u8,
    finetune: u32,
    loop_length: u32,
    sample_start: u32,
    sample_end: u32,
}

impl AssetChunk for TSample {
    fn parse(input: &[u8]) -> Result<Self> {
        let (input, flags) = number::le_u8(input)?;
        let (input, volume) = number::le_u8(input)?;
        let (input, panning) = number::le_u8(input)?;
        let (input, align) = number::le_u8(input)?;
        let (input, finetune) = number::le_u32(input)?;
        let (input, loop_length) = number::le_u32(input)?;
        let (input, loop_end) = number::le_u32(input)?;
        let (input, sample) = number::le_u32(input)?;

        Ok((
            input,
            Self {
                flags,
                volume,
                panning,
                align,
                finetune,
                loop_length,
                sample_start: loop_end,
                sample_end: sample,
            },
        ))
    }
}
