use std::{
    fmt::Debug,
    io::{Cursor, Read, Seek, SeekFrom},
};

use derive_more::{Deref, From};
use flate2::read::ZlibDecoder;

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

        let mut decoder = ZlibDecoder::new(&mut *reader);
        let mut decompressed = Vec::with_capacity(header.size_decompressed as usize);
        let size_compressed = decoder.read_to_end(&mut decompressed)?;

        if decompressed.len() != header.size_decompressed as usize {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "zlib decompression resulted into invalid size",
            )));
        }

        reader.seek(SeekFrom::Start(pos + size_compressed as u64))?;
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
        let mut decompressed = Cursor::new(vec![]);
        self.0.write_options(&mut decompressed, endian, args)?;
        let decompressed = decompressed.into_inner();

        ZlibHeader {
            size_decompressed: decompressed.len() as u32,
        }
        .write_options(writer, endian, ())?;
        decompressed.write_options(writer, endian, ())
    }
}
