use core::cell::Cell;
use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian, NamedArgs, binread, binrw, helpers::args_iter};
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

#[derive(Debug, Clone)]
struct PosMarker<T> {
    pos: Cell<u64>,
    value: T,
}

impl<T> BinRead for PosMarker<T>
where
    T: BinRead,
{
    type Args<'a> = T::Args<'a>;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let pos = reader.stream_position()?;
        T::read_options(reader, endian, args).map(|value| Self {
            pos: Cell::new(pos),
            value,
        })
    }
}

impl<T> BinWrite for PosMarker<T>
where
    T: BinWrite<Args<'static> = ()> + Default,
{
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        self.pos.set(writer.stream_position()?);
        T::default().write_options(writer, endian, args)
    }
}

fn args_iter_write<'a, T, Writer, Arg, It>(
    it: It,
) -> impl FnOnce(&T, &mut Writer, Endian, ()) -> BinResult<()>
where
    T: BinWrite<Args<'a> = Arg>,
    Writer: Write + Seek,
    Arg: Clone,
    It: IntoIterator<Item = Arg> + Clone,
{
    move |current, writer, endian, ()| {
        it.clone()
            .into_iter()
            .map(|arg| T::write_options(current, writer, endian, arg.clone()))
            .collect()
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
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
fn pack_file_entry_writer(this: &Vec<u8>, header: &PackFileEntryHeader) -> BinResult<()> {
    let pos = writer.stream_position()?;

    writer.seek(SeekFrom::Start(header.offset.pos.get() as u64));
    (pos as u32).write_options(writer, endian, ())?;

    writer.seek(SeekFrom::Start(pos));
    this.write_options(writer, endian, ());

    Ok(())
}

#[binrw]
#[brw(import_raw(header: PackFileEntryHeader))]
#[derive(Debug)]
pub struct PackFileEntry(
    #[br(seek_before = SeekFrom::Start(header.offset.value as u64), count = header.size.value)]
    #[bw(write_with = pack_file_entry_writer, args(&header))]
    Vec<u8>,
);

// [Related Issue](https://github.com/jam1garner/binrw/discussions/165#discussioncomment-5729414)
#[binrw]
#[brw(little, magic = b"PMAN")]
#[derive(Debug)]
pub struct PackFile {
    #[br(temp)]
    #[bw(calc = entries.len() as u32)]
    _entries_count: u32,

    #[brw(args { len: 56 })]
    copyright: PaddedNullString,

    #[br(count = _entries_count)]
    _entries: Vec<PackFileEntryHeader>,

    #[br(parse_with = args_iter(_entries.clone()))]
    #[bw(write_with = args_iter_write(_entries.clone()))]
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

        // dbg!(rom);

        Ok(())
    }
}
