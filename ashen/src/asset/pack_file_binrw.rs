use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{
    BinRead, BinResult, BinWrite, Endian, NamedArgs, binread, binrw, helpers::args_iter_with,
};
use itertools::Itertools;

#[derive(NamedArgs, Clone)]
struct PaddedNullStringArgs {
    len: usize,
}

#[derive(Debug)]
struct PaddedNullString(String);

impl BinRead for PaddedNullString {
    type Args<'a> = PaddedNullStringArgs;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let value: Vec<u8> =
            binrw::helpers::count_with(args.len, u8::read_options)(reader, endian, ())?;

        let value = String::from_utf8_lossy(&value)
            .trim_end_matches('\0')
            .to_string();

        Ok(Self(value))
    }
}

impl BinWrite for PaddedNullString {
    type Args<'a> = PaddedNullStringArgs;

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let bytes = self.0.as_bytes();

        if bytes.len() >= args.len {
            return Err(binrw::Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "string too large to fit into specified length",
            )));
        }

        bytes.write_options(writer, endian, ())?;
        std::iter::repeat(0u8)
            .take(args.len - bytes.len())
            .collect_vec()
            .write_options(writer, endian, ())?;

        Ok(())
    }
}

#[binrw]
#[br(little)]
#[derive(Debug, Clone)]
pub struct PackFileEntryHeader {
    #[br(temp, assert(_asset_kind == 0))]
    #[bw(calc(0))]
    _asset_kind: u32,

    #[br(map = |x: u32| x as usize)]
    #[bw(map = |&x| x as u32)]
    offset: usize,

    #[br(map = |x: u32| x as usize)]
    #[bw(map = |&x| x as u32)]
    size: usize,

    #[br(temp, assert(_reserved == 0))]
    #[bw(calc(0))]
    _reserved: u32,
}

pub fn pack_file_entry_header_parser<R>(
    headers: &[PackFileEntryHeader],
) -> impl FnOnce(&mut R, Endian, ()) -> BinResult<Vec<Vec<u8>>>
where
    R: Read + Seek,
{
    move |reader, endian, ()| {
        let before = reader.stream_position()?;
        reader.seek(SeekFrom::Start(0))?;

        let entries = headers
            .iter()
            .map(|h| -> BinResult<Vec<u8>> {
                let before = reader.stream_position()?;
                reader.seek(SeekFrom::Current(h.offset as i64))?;

                let entry = Vec::read_options(reader, endian, binrw::args! { count: h.size })?;

                reader.seek(SeekFrom::Start(before))?;
                Ok(entry)
            })
            .collect::<Result<_, _>>()?;

        reader.seek(SeekFrom::Start(before))?;
        Ok(entries)
    }
}

#[binread]
#[br(little, magic = b"PMAN")]
#[derive(Debug)]
pub struct PackFile {
    #[br(temp)]
    // #[bw(calc = entries.len() as u32)]
    _entries_count: u32,

    #[brw(args { len: 56 })]
    copyright: PaddedNullString,

    #[br(temp, count = _entries_count)]
    _entries_headers: Vec<PackFileEntryHeader>,

    #[br(parse_with = pack_file_entry_header_parser(&_entries_headers))]
    entries: Vec<Vec<u8>>,
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

        rom.entries.iter().zip(pack_file.entries.iter()).for_each(|(n, o)| {
            assert_eq!(n, &o.bytes);
        });

        // dbg!(rom);

        Ok(())
    }
}
