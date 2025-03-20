use binrw::{BinRead, BinWrite, NamedArgs, binread, binrw};
use itertools::Itertools;

#[derive(NamedArgs, Clone)]
struct PaddedNullStringArgs {
    len: usize,
}

#[derive(Debug)]
struct PaddedNullString(String);

impl BinRead for PaddedNullString {
    type Args<'a> = PaddedNullStringArgs;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
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

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
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
#[derive(Debug)]
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

        dbg!(rom);

        Ok(())
    }
}
