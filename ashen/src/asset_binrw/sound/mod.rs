use std::io::SeekFrom;

use super::utils::*;

#[binrw]
#[derive(Debug, Clone, Default)]
pub struct EntryHeader {
    offset: Marker<u32>,

    size: Marker<u32>,

    #[br(temp, assert(_reserved == 0))]
    #[bw(calc(0))]
    _reserved: u32,
}

#[binrw]
#[derive(Debug)]
pub struct Songs {
    #[br(temp)]
    #[bw(calc = songs.len() as u32)]
    _headers_len: u32,

    #[br(temp, count = _headers_len)]
    #[bw(calc(vec![Default::default(); songs.len()]))]
    _headers: Vec<EntryHeader>,

    #[br(parse_with = args_iter(_headers))]
    #[bw(write_with = args_iter_write(&_headers))]
    songs: Vec<CompressedTSongs>,
}

#[derive(Debug)]
pub struct CompressedTSongs(TSong);

impl BinRead for CompressedTSongs {
    type Args<'a> = EntryHeader;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: Endian,
        header: Self::Args<'_>,
    ) -> BinResult<Self> {
        reader.seek(SeekFrom::Start(header.offset.value as u64))?;
        let sound =
            <Compressed<TSong>>::read_options(reader, endian, ()).map(Compressed::into_inner)?;
        Ok(Self(sound))
    }
}

impl BinWrite for CompressedTSongs {
    type Args<'a> = &'a EntryHeader;

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        todo!()
    }
}

#[binrw]
#[derive(Debug)]
pub struct TSong {}

#[binrw]
#[brw(magic = b"TSND")]
#[derive(Debug)]
pub struct Sounds {
    _songs_header: EntryHeader,
    _effects_header: EntryHeader,
    _maps_header: EntryHeader,
    _emitters_header: EntryHeader,

    #[br(
        seek_before = SeekFrom::Start(_songs_header.offset.value as u64),
        pad_size_to = _songs_header.size.value as u64,
    )]
    songs: Songs,

    #[br(
        seek_before = SeekFrom::Start(_effects_header.offset.value as u64),
        pad_size_to = _effects_header.size.value as u64,
    )]
    effects: (),

    #[br(
        seek_before = SeekFrom::Start(_maps_header.offset.value as u64),
        pad_size_to = _maps_header.size.value as u64,
    )]
    maps: (),

    #[br(
        seek_before = SeekFrom::Start(_emitters_header.offset.value as u64),
        pad_size_to = _emitters_header.size.value as u64,
    )]
    emitters: (),
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io::Cursor};

    use super::*;
    use crate::utils::test::*;

    const SOUND_DATA: LazyCell<Vec<u8>> = deflated_file!("97.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let sounds = Sounds::read_le(&mut Cursor::new(SOUND_DATA.as_slice()))?;
        Ok(())
    }

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn write_rom_asset() -> eyre::Result<()> {
        let sounds = Sounds::read_le(&mut Cursor::new(SOUND_DATA.as_slice()))?;
        let output = {
            let mut output = Cursor::new(vec![]);
            sounds.write_le(&mut output)?;
            output.into_inner()
        };
        dbg!(sounds);
        //assert_eq!(*SOUND_DATA, output);
        Ok(())
    }
}
