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
pub struct SoundsEntryHeaders {
    #[br(temp)]
    #[bw(calc = entries.len() as u32)]
    _entries_len: u32,

    #[br(temp, count = _entries_len)]
    #[bw(calc(vec![Default::default(); entries.len()]))]
    _entries: Vec<EntryHeader>,

    #[br(parse_with = args_iter(_entries))]
    #[bw(write_with = args_iter_write(&_entries))]
    entries: Vec<TSound>,
}

#[binrw]
#[br(import_raw(header: EntryHeader))]
#[bw(import_raw(header: &EntryHeader))]
#[derive(Debug)]
pub struct TSound {

}

#[binrw]
#[brw(magic = b"TSND")]
#[derive(Debug)]
pub struct Sounds {
    _songs_header: EntryHeader,
    _effects_header: EntryHeader,
    _maps_header: EntryHeader,
    _emitters_header: EntryHeader,

    songs: SoundsEntryHeaders,
    effects: (),
    maps: (),
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
