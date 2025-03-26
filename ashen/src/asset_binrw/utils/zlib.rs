use std::{
    fmt::Debug,
    io::{BufReader, Cursor, Read, Seek, SeekFrom, Write},
};

use derive_more::{Deref, From};
use flate2::{Compression, bufread::ZlibDecoder, write::ZlibEncoder};

use super::*;

#[binrw]
#[brw(magic = b"ZL")]
struct ZlibHeader {
    #[br(parse_with = read_u24)]
    #[bw(write_with = write_u24)]
    size_decompressed: u32,
}

#[derive(Deref, From)]
struct Compressed<T>(T);

impl<T> Debug for Compressed<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Compressed").field(&self.0).finish()
    }
}

impl<T, Arg> BinRead for Compressed<T>
where
    T: for<'a> BinRead<Args<'a> = Arg> + for<'a> BinWrite<Args<'a> = Arg>,
{
    type Args<'a> = Arg;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let header = ZlibHeader::read_options(reader, endian, ())?;
        let pos = reader.stream_position()?;

        let mut decoder = ZlibDecoder::new(BufReader::new(&mut *reader));
        let mut decompressed = Vec::with_capacity(header.size_decompressed as usize);
        decoder.read_to_end(&mut decompressed)?;

        if decompressed.len() != header.size_decompressed as usize {
            return Err(Error::AssertFail {
                pos,
                message: format!(
                    "zlib decompression expected to produce a file of size {} but got {}",
                    header.size_decompressed,
                    decompressed.len()
                ),
            });
        }

        let pos = decoder.into_inner().stream_position()?;
        reader.seek(SeekFrom::Start(pos))?;
        Ok(Self(T::read_options(
            &mut Cursor::new(decompressed),
            endian,
            args,
        )?))
    }
}

impl<T, Arg> BinWrite for Compressed<T>
where
    T: for<'a> BinWrite<Args<'a> = Arg>,
{
    type Args<'a> = Arg;

    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let mut data = Cursor::new(vec![]);
        self.0.write_options(&mut data, endian, args)?;
        let size_decompressed = data.stream_position()?;

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&data.into_inner())?;
        let compressed = encoder.finish()?;

        ZlibHeader {
            size_decompressed: size_decompressed as u32,
        }
        .write_options(writer, endian, ())?;
        compressed.write_options(writer, endian, ())
    }
}

#[binrw]
#[derive(Debug)]
struct EntireFile {
    #[br(parse_with = until_eof)]
    data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, io::Cursor};

    use super::*;
    use crate::{asset_binrw::pack_file::PackFile, utils::test::*};

    const COMPRESSED_DATA: LazyCell<Vec<u8>> = deflated_file!("0A.dat");
    const UNCOMPRESSED_DATA: LazyCell<Vec<u8>> = deflated_file!("0A-deflated.dat");

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn parse_rom_asset() -> eyre::Result<()> {
        let compressed =
            <Compressed<EntireFile>>::read_le(&mut Cursor::new(COMPRESSED_DATA.as_slice()))?;

        Ok(())
    }

    const ROM_DATA: LazyCell<Vec<u8>> = std::cell::LazyCell::new(|| {
        std::fs::read(workspace_file_path!("rom/packfile.dat")).expect("ROM is present")
    });

    #[test]
    #[ignore = "uses Ashen ROM files"]
    fn write_file() -> eyre::Result<()> {
        let mut rom = PackFile::read(&mut Cursor::new(ROM_DATA.as_slice()))?;

        for i in (0x0A..=0x3B) {
            let model = rom.entries[i].0.clone();
            let uncompressed = <Compressed<EntireFile>>::read_le(&mut Cursor::new(model))?;

            let mut recompressed = Cursor::new(vec![]);
            uncompressed.write_le(&mut recompressed)?;

            rom.entries[i].0 = recompressed.into_inner();
        }

        let mut output = Cursor::new(vec![]);
        rom.write(&mut output);

        std::fs::write(
            workspace_file_path!("rom/packfile.new-model.dat"),
            output.into_inner(),
        )?;

        Ok(())
    }
}
