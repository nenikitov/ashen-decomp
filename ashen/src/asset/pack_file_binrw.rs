use std::io::SeekFrom;

use crate::utils::binrw::*;

const LEN_COPYRIGHT: usize = 56;

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, Default)]
pub struct PackFileEntryHeader {
    #[br(temp, assert(_asset_kind == 0))]
    #[bw(calc(0))]
    _asset_kind: u32,

    offset: PosMarker<u32>,

    size: PosMarker<u32>,

    #[br(temp, assert(_reserved == 0))]
    #[bw(calc(0))]
    _reserved: u32,
}

#[binrw::writer(writer, endian)]
fn pack_file_entry_write_and_update_header(
    this: &Vec<u8>,
    header: &PackFileEntryHeader,
) -> BinResult<()> {
    let start = writer.stream_position()?;

    writer.seek(SeekFrom::Start(header.offset.pos.get() as u64))?;
    (start as u32).write_options(writer, endian, ())?;

    writer.seek(SeekFrom::Start(header.size.pos.get() as u64))?;
    (this.len() as u32).write_options(writer, endian, ())?;

    writer.seek(SeekFrom::Start(start))?;
    this.write_options(writer, endian, ())
}

#[binrw]
#[br(import_raw(header: PackFileEntryHeader))]
#[bw(import_raw(header: &PackFileEntryHeader))]
#[derive(Debug)]
pub struct PackFileEntry(
    #[br(seek_before = SeekFrom::Start(header.offset.value as u64), count = header.size.value)]
    #[bw(write_with = pack_file_entry_write_and_update_header, args(header))]
    Vec<u8>,
);

#[binrw]
#[brw(little, magic = b"PMAN")]
#[derive(Debug)]
pub struct PackFile {
    #[br(temp)]
    #[bw(calc = entries.len() as u32)]
    _entries_count: u32,

    #[brw(args { len: LEN_COPYRIGHT })]
    #[br(map = PaddedNullString::into)]
    #[bw(map = |x| PaddedNullString::from(x.to_owned()))]
    copyright: String,

    #[br(temp, count = _entries_count)]
    #[bw(calc(vec![Default::default(); entries.len()]))]
    _entries: Vec<PackFileEntryHeader>,

    #[br(parse_with = args_iter(_entries))]
    #[bw(write_with = args_iter_write(&_entries))]
    entries: Vec<PackFileEntry>,
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io::Cursor};

    use super::*;
    use crate::utils::test::*;

    const ROM_DATA: LazyCell<Vec<u8>> = std::cell::LazyCell::new(|| {
        std::fs::read(workspace_file_path!("rom/packfile.dat")).expect("ROM is present")
    });

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let rom = PackFile::read(&mut Cursor::new(ROM_DATA.as_slice()))?;
        let (_, pack_file) = crate::asset::pack_file::PackFile::new(&ROM_DATA)?;

        rom.entries
            .iter()
            .zip(pack_file.entries.iter())
            .for_each(|(n, o)| {
                assert_eq!(&n.0, &o.bytes);
            });

        let mut output = Cursor::new(vec![]);
        rom.write(&mut output);

        std::fs::write(
            workspace_file_path!("rom/packfile.new.dat"),
            output.into_inner(),
        )?;

        Ok(())
    }
}
